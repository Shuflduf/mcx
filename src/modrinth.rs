use crate::{
    config::{self, LoaderName, MCXConfig, ModInfo},
    mods,
};
use chrono::{DateTime, Utc};
use color_eyre::{
    eyre::{eyre, Result},
    owo_colors::OwoColorize,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ProjectInfo {
    title: String,
    slug: String,
}

#[derive(Debug, Deserialize)]
struct ModFile {
    url: String,
    #[allow(dead_code)]
    size: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum DependencyType {
    Required,
    Optional,
    Incompatible,
    Embedded,
}

#[derive(Debug, Deserialize)]
struct ModDependency {
    project_id: String,
    dependency_type: DependencyType,
}

#[derive(Debug, Deserialize)]
struct ModVersion {
    date_published: DateTime<Utc>,
    files: Vec<ModFile>,
    dependencies: Vec<ModDependency>,
}

pub fn update_all_mods() -> Result<()> {
    let conf = config::get_config()?;
    let mods_list = conf.mods.unwrap_or_default();
    let version_info = conf.version_info;
    for target_mod in mods_list {
        let req_url = urls::list_project_versions(
            &target_mod.id,
            &version_info.name,
            &version_info.game_version,
        );
        let mod_versions: Vec<ModVersion> =
            serde_json::from_str(&reqwest::blocking::get(req_url)?.text()?)?;
        if mod_versions.is_empty() {
            continue;
        }
        if mod_versions[0].date_published <= target_mod.version_date {
            continue;
        }
        println!(
            "NEW {} > {}",
            mod_versions[0].date_published, target_mod.version_date
        );
        // this doesnt do deps stuff and i think it should
        mods::download_mod_jar(&mod_versions[0].files[0].url, &target_mod.id)?;
        config::update_mod(&target_mod.id, &mod_versions[0].date_published)?;
    }
    Ok(())
}

pub fn download_from_id(id: &str, dependency_level: usize) -> Result<()> {
    let version_info = config::get_version_info()?;
    if version_info.name == LoaderName::Vanilla {
        return Err(eyre!("Mods are not supported for Vanilla"));
    }
    let project_info = get_mod_info(id)?;
    if config::has_mod(&project_info.slug)? {
        if dependency_level == 0 {
            return Err(eyre!("Mod \"{}\" already installed", project_info.title));
        }
        return Ok(());
    }
    println!(
        "  {}{} {}",
        "  ".repeat(dependency_level),
        "Downloading".bold().green(),
        project_info.title
    );
    let req_url = urls::list_project_versions(
        &project_info.slug,
        &version_info.name,
        &version_info.game_version,
    );
    let modrinth_response = reqwest::blocking::get(&req_url)?.text()?;
    let versions: Vec<ModVersion> = serde_json::from_str(&modrinth_response)?;
    if versions.is_empty() {
        return Err(eyre!(
            "Mod \"{}\" not found for {:?} {}",
            project_info.title,
            version_info.name,
            version_info.game_version
        ));
    }
    let target_version = &versions[0];
    for dep in &target_version.dependencies {
        if dep.dependency_type == DependencyType::Required
            || (dep.dependency_type == DependencyType::Optional
                && inquire::Confirm::new(&format!(
                    "Install optional dependency \"{}\"",
                    get_mod_info(&dep.project_id)?.title
                ))
                .with_default(true)
                .prompt()?)
        {
            download_from_id(&dep.project_id, dependency_level + 1)?;
        }
    }
    mods::download_mod_jar(&target_version.files[0].url, &project_info.slug)?;
    let mod_info = ModInfo {
        name: project_info.title,
        id: project_info.slug,
        version_date: target_version.date_published,
    };
    config::add_mod(mod_info)?;
    // println!(
    //     "modrinth {:#?}",
    //     versions
    //         .iter()
    //         .map(|v| v.files[0].url.clone())
    //         .collect::<Vec<_>>()
    // );
    Ok(())
}

fn get_mod_info(id: &str) -> Result<ProjectInfo> {
    let url = urls::get_project_info(id);
    let resp = reqwest::blocking::get(&url)?;
    if let Err(e) = resp.error_for_status_ref() {
        if e.status() == Some(reqwest::StatusCode::NOT_FOUND) {
            return Err(eyre!("Mod \"{}\" does not exist", id));
        }
        return Err(e.into());
    }
    let project_info: ProjectInfo = serde_json::from_str(&resp.text()?)?;
    Ok(project_info)
}

mod urls {
    use crate::config::LoaderName;

    pub fn list_project_versions(
        slug: &str,
        loader_name: &LoaderName,
        game_version: &str,
    ) -> String {
        format!(
            "https://api.modrinth.com/v2/project/{slug}/version?loaders=[\"{}\"]&game_versions=[\"{game_version}\"]",
            format!("{:?}", loader_name).to_lowercase(),
        )
    }
    pub fn get_project_info(slug: &str) -> String {
        format!("https://api.modrinth.com/v2/project/{slug}")
    }
}
