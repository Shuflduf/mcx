use crate::config::{self, LoaderName};
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
    let modrinth_response = reqwest::blocking::get(req_url)?.text()?;
    let versions: Vec<ModVersion> = serde_json::from_str(&modrinth_response)?;
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
}
