use color_eyre::eyre::Result;
use serde::Deserialize;

use crate::loaders::{include_snapshots, MCLoader};

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
        println!("{}", loader_versions_list[0].version);

        Ok(())
    }
}

fn filter_snapshots(versions: &mut Vec<GameVersion>, include_snapshots: bool) {
    versions.retain_mut(|v| include_snapshots || v.stable);
}

mod version_getter {
    use color_eyre::eyre::Result;
    use serde::Deserialize;

    use crate::loaders::fabric::GameVersion;

    #[derive(Deserialize)]
    struct FabricResponse {
        versions: Vec<GameVersion>,
    }

    pub fn get_mc_versions() -> Result<Vec<GameVersion>> {
        let url = "https://meta.fabricmc.net/v2/versions/game";
        let raw_response = reqwest::blocking::get(url)?.text()?;
        println!("{raw_response}");
        let response: FabricResponse = serde_json::from_str(&raw_response)?;
        Ok(response.versions)
    }

    pub fn get_loader_versions() -> Result<Vec<GameVersion>> {
        let url = "https://meta.fabricmc.net/v2/versions/loader";
        let raw_response = reqwest::blocking::get(url)?.text()?;
        let response: FabricResponse = serde_json::from_str(&raw_response)?;
        Ok(response.versions)
    }
}
