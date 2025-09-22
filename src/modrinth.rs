use crate::config::{self, LoaderName, ModInfo};
use chrono::{DateTime, Utc};
use color_eyre::eyre::{eyre, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct ModFile {
    url: String,
    filename: String,
    #[allow(dead_code)]
    size: u32,
}

#[derive(Deserialize)]
struct ModVersion {
    date_published: DateTime<Utc>,
    files: Vec<ModFile>,
}

// also supports downloading from project id
pub fn download_from_slug(slug: &str) -> Result<()> {
    let version_info = config::get_version_info()?;
    if version_info.name == LoaderName::Vanilla {
        return Err(eyre!("Mods are not supported for Vanilla"));
    }
    let req_url = format!(
        "{}?loaders=[\"{}\"]&game_versions=[\"{}\"]",
        urls::list_project_versions(slug),
        "fabric",
        "1.20.1"
    );
    // println!("{:?}", reqwest::blocking::get(&req_url));
    let resp = reqwest::blocking::get(&req_url)?;
    if let Err(e) = resp.error_for_status_ref() {
        if e.status() == Some(reqwest::StatusCode::NOT_FOUND) {
            return Err(eyre!("Mod \"{}\" not found", slug));
        }
        return Err(e.into());
    }
    let modrinth_response = resp.text()?;
    println!("{req_url}");
    let versions: Vec<ModVersion> = serde_json::from_str(&modrinth_response)?;
    // let target_version = &versions[0];
    let version_info = ModInfo {
        name: "Test".into(),
        slug: Some("test".into()),
        version_date: None,
    };
    config::add_mod(version_info)?;
    println!(
        "modrinth {:?}",
        versions
            .iter()
            .map(|v| v.files[0].url.clone())
            .collect::<Vec<_>>()
    );
    Ok(())
}

mod urls {
    pub fn list_project_versions(slug: &str) -> String {
        format!("https://api.modrinth.com/v2/project/{slug}/version")
    }
    pub fn get_project_info(slug: &str) -> String {
        format!("https://api.modrinth.com/v2/project/{slug}")
    }
}
