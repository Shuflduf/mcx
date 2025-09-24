use crate::{
    config::{self, LoaderName, ModInfo},
    mods,
};
use chrono::{DateTime, Utc};
use color_eyre::{
    eyre::{eyre, Result},
    owo_colors::OwoColorize,
};
use serde::Deserialize;

// TODO: make pack.rs a module of modrinth and stop making everything pub

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectType {
    Mod,
    ModPack,
    ResourcePack,
    Shader,
}

#[derive(Debug, Deserialize)]
pub struct ProjectInfo {
    pub title: String,
    pub slug: String,
    pub project_type: ProjectType,
    pub game_versions: Vec<String>,
    pub loaders: Vec<LoaderName>,
}

#[derive(Debug, Deserialize)]
pub struct ModFile {
    pub url: String,
    #[allow(dead_code)]
    pub size: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DependencyType {
    Required,
    Optional,
    Incompatible,
    Embedded,
}

#[derive(Debug, Deserialize)]
pub struct ModDependency {
    pub project_id: String,
    pub dependency_type: DependencyType,
}

#[derive(Debug, Deserialize)]
pub struct ProjectVersion {
    pub date_published: DateTime<Utc>,
    pub files: Vec<ModFile>,
    pub dependencies: Vec<ModDependency>,
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
        let mod_versions: Vec<ProjectVersion> =
            serde_json::from_str(&reqwest::blocking::get(req_url)?.text()?)?;
        if mod_versions.is_empty() {
            continue;
        }
        if mod_versions[0].date_published <= target_mod.version_date {
            continue;
        }
        println!("  {} {}", "Updating".bold().green(), target_mod.name);
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
    let project_info = get_project_info(id)?;
    if config::has_mod(&project_info.slug)? {
        if dependency_level == 0 {
            return Err(eyre!("Mod \"{}\" already installed", project_info.title));
        }
        return Ok(());
    }
    if project_info.project_type != ProjectType::Mod {
        return Err(eyre!("Project \"{}\" is not a mod", project_info.title));
    }
    println!(
        "  {}{} {}",
        "  ".repeat(dependency_level),
        "Downloading".bold().green(),
        project_info.title
    );
    let versions = get_project_versions(
        &project_info.slug,
        &version_info.name,
        &version_info.game_version,
    )?;
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
                    get_project_info(&dep.project_id)?.title
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

pub fn get_project_versions(
    id: &str,
    loader: &LoaderName,
    game_version: &str,
) -> Result<Vec<ProjectVersion>> {
    let req_url = urls::list_project_versions(id, loader, game_version);
    let modrinth_response = reqwest::blocking::get(&req_url)?.text()?;
    let versions: Vec<ProjectVersion> = serde_json::from_str(&modrinth_response)?;
    Ok(versions)
}

pub fn get_project_info(id: &str) -> Result<ProjectInfo> {
    let url = urls::get_project_info(id);
    let resp = reqwest::blocking::get(&url)?;
    if let Err(e) = resp.error_for_status_ref() {
        if e.status() == Some(reqwest::StatusCode::NOT_FOUND) {
            return Err(eyre!("Mod \"{}\" does not exist", id));
        }
        return Err(e.into());
    }
    let json_data = resp.text()?;
    let project_info: ProjectInfo = serde_json::from_str(&json_data)?;
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
