use crate::{
    config::{self, VersionInfo},
    init,
    loaders::MCLoader,
};
use color_eyre::eyre::{OptionExt, Result};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
struct GameVersion {
    id: String,
    #[serde(rename = "type")]
    version_type: String,
    url: String,
}

#[derive(Default)]
pub struct VanillaLoader {
    versions_list: Vec<GameVersion>,
    version: String,
}

impl MCLoader for VanillaLoader {
    fn setup_versions(&mut self) -> Result<()> {
        self.versions_list = version_getter::get_mc_versions()?;
        filter_snapshots(&mut self.versions_list, include_snapshots()?);
        self.version = inquire::Select::new(
            "Version",
            self.versions_list.iter().map(|v| v.id.clone()).collect(),
        )
        .prompt()?;
        config::create_config(VersionInfo {
            name: config::LoaderName::Vanilla,
            game_version: self.version.clone(),
            loader_version: None,
        })?;
        Ok(())
    }
    fn download_server_jar(&mut self) -> Result<()> {
        let metadata_url = &self
            .versions_list
            .iter()
            .find(|v| v.id == self.version)
            .ok_or_eyre("Server JAR URL not found")?
            .url;
        println!("{metadata_url}");
        let jar_url = jar_url_getter::get_jar_url(metadata_url.to_string())?;
        println!("{jar_url}");
        init::download_server_file(jar_url)?;
        Ok(())
    }
}

fn include_snapshots() -> Result<bool> {
    Ok(inquire::Confirm::new("Include Snapshots?")
        .with_default(false)
        .prompt()?)
}

fn filter_snapshots(versions: &mut Vec<GameVersion>, include_snapshots: bool) {
    versions.retain_mut(|v| include_snapshots || v.version_type == "release");
}

mod version_getter {
    use color_eyre::eyre::Result;
    use serde::Deserialize;

    use crate::loaders::vanilla::GameVersion;

    #[derive(Deserialize)]
    struct MojangResponse {
        versions: Vec<GameVersion>,
    }

    pub fn get_mc_versions() -> Result<Vec<GameVersion>> {
        let url = "https://launchermeta.mojang.com/mc/game/version_manifest.json";
        let raw_response = reqwest::blocking::get(url)?.text()?;
        let response: MojangResponse = serde_json::from_str(&raw_response)?;
        Ok(response.versions)
    }
}
mod jar_url_getter {
    use color_eyre::eyre::Result;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct JarInfo {
        #[allow(dead_code)]
        size: u32,
        url: String,
    }
    #[derive(Deserialize)]
    struct JarUrls {
        server: JarInfo,
    }
    #[derive(Deserialize)]
    struct VersionMetadata {
        downloads: JarUrls,
    }

    pub fn get_jar_url(metadata_url: String) -> Result<String> {
        let raw_response = reqwest::blocking::get(metadata_url)?.text()?;
        let response: VersionMetadata = serde_json::from_str(&raw_response)?;
        Ok(response.downloads.server.url)
    }
}
