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

#[derive(Debug, Deserialize)]
struct ProjectInfo {
    title: String,
}

#[derive(Debug, Deserialize)]
struct ModFile {
    url: String,
    #[allow(dead_code)]
    size: u32,
}

#[derive(Debug, Deserialize)]
struct ModVersion {
    date_published: DateTime<Utc>,
    files: Vec<ModFile>,
}

// also supports downloading from project id
pub fn download_from_slug(slug: &str, dependency_level: usize) -> Result<()> {
    let version_info = config::get_version_info()?;
    if version_info.name == LoaderName::Vanilla {
        return Err(eyre!("Mods are not supported for Vanilla"));
    }
    if config::has_mod(slug)? {
        if dependency_level == 0 {
            return Err(eyre!("Mod \"{slug}\" already installed"));
        }
        return Ok(());
    }
    let req_url = format!(
        "{}?loaders=[\"{}\"]&game_versions=[\"{}\"]",
        urls::list_project_versions(slug),
        format!("{:?}", version_info.name).to_lowercase(),
        version_info.game_version
    );
    // println!("{:?}", reqwest::blocking::get(&req_url));
    let resp = reqwest::blocking::get(&req_url)?;
    if let Err(e) = resp.error_for_status_ref() {
        if e.status() == Some(reqwest::StatusCode::NOT_FOUND) {
            return Err(eyre!("Mod \"{}\" does not exist", slug));
        }
        return Err(e.into());
    }
    let modrinth_response = resp.text()?;
    println!("{req_url}");
    let versions: Vec<ModVersion> = serde_json::from_str(&modrinth_response)?;
    if versions.is_empty() {
        return Err(eyre!(
            "Mod \"{}\" not found for {:?} {}",
            slug,
            version_info.name,
            version_info.game_version
        ));
    }
    let target_version = &versions[0];
    let mod_name = get_mod_name(slug)?;
    println!(
        "  {}{} {mod_name}",
        "  ".repeat(dependency_level),
        "Downloading".bold().green()
    );
    mods::download_mod_jar(&target_version.files[0].url, slug)?;
    let mod_info = ModInfo {
        name: mod_name,
        id: slug.into(),
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

fn get_mod_name(slug: &str) -> Result<String> {
    let url = urls::get_project_info(slug);
    let resp = reqwest::blocking::get(url)?.text()?;
    let project_info: ProjectInfo = serde_json::from_str(&resp)?;
    Ok(project_info.title)
}

mod urls {
    pub fn list_project_versions(slug: &str) -> String {
        format!("https://api.modrinth.com/v2/project/{slug}/version")
    }
    pub fn get_project_info(slug: &str) -> String {
        format!("https://api.modrinth.com/v2/project/{slug}")
    }
}
