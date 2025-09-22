use color_eyre::eyre::Result;
use serde::Deserialize;

use crate::{
    config::{self, VersionInfo},
    init,
    loaders::{include_snapshots, MCLoader},
};

#[derive(Deserialize)]
struct GameVersion {
    version: String,
    stable: bool,
}

#[derive(Default)]
pub struct FabricLoader {
    game_version: String,
    loader_version: String,
}

impl MCLoader for FabricLoader {
    fn setup_versions(&mut self) -> Result<()> {
        let mut game_versions_list = version_getter::get_mc_versions()?;
        filter_snapshots(&mut game_versions_list, include_snapshots()?);
        let game_versions = game_versions_list
            .iter()
            .map(|v| v.version.clone())
            .collect();
        self.game_version = inquire::Select::new("Version", game_versions).prompt()?;

        let loader_versions_list = version_getter::get_loader_versions()?;
        let loader_versions = loader_versions_list
            .iter()
            .map(|v| v.version.clone())
            .collect();

        self.loader_version = inquire::Select::new("Loader Version", loader_versions).prompt()?;
        config::create_config(VersionInfo {
            name: config::LoaderName::Fabric,
            game_version: self.game_version.clone(),
            loader_version: Some(self.loader_version.clone()),
        })?;
        Ok(())
    }
    fn download_server_jar(&mut self) -> Result<()> {
        let jar_url = format!(
            "https://meta.fabricmc.net/v2/versions/loader/{}/{}/{}/server/jar",
            self.game_version, self.loader_version, "1.1.0"
        );
        println!("{jar_url}");
        init::download_server_file(jar_url)?;
        Ok(())
    }
}

fn filter_snapshots(versions: &mut Vec<GameVersion>, include_snapshots: bool) {
    versions.retain_mut(|v| include_snapshots || v.stable);
}

mod version_getter {
    use crate::loaders::fabric::GameVersion;
    use color_eyre::eyre::Result;

    fn make_fabric_req(url: &str) -> Result<Vec<GameVersion>> {
        let raw_response = reqwest::blocking::get(url)?.text()?;
        Ok(serde_json::from_str(&raw_response)?)
    }

    pub fn get_mc_versions() -> Result<Vec<GameVersion>> {
        let url = "https://meta.fabricmc.net/v2/versions/game";
        make_fabric_req(url)
    }

    pub fn get_loader_versions() -> Result<Vec<GameVersion>> {
        let url = "https://meta.fabricmc.net/v2/versions/loader";
        make_fabric_req(url)
    }
}
