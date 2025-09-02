use crate::loaders::{vanilla::version_getter::GameVersion, MCLoader};
use color_eyre::eyre::Result;

#[derive(Debug)]
pub struct VanillaLoader {}

impl MCLoader for VanillaLoader {
    fn tmp(&self) {
        println!("vanilla gaming")
    }
    fn get_mc_versions(&self) -> Result<Vec<String>> {
        version_getter::get_mc_versions()
    }
}

fn filter_snapshots(versions: Vec<GameVersion>) -> Vec<GameVersion> {
    versions
        .iter()
        .filter(|v| v.version_type == "release")
        .collect()
}

mod version_getter {
    use color_eyre::eyre::Result;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct MojangResponse {
        versions: Vec<GameVersion>,
    }

    #[derive(Deserialize)]
    pub struct GameVersion {
        pub id: String,
        #[serde(rename = "type")]
        pub version_type: String,
        pub url: String,
    }

    pub fn get_mc_versions() -> Result<Vec<GameVersion>> {
        let url = "https://launchermeta.mojang.com/mc/game/version_manifest.json";
        let raw_response = reqwest::blocking::get(url)?;
        // let parsed
        let response: MojangResponse = serde_json::from_str(&raw_response.text()?)?;
        // Ok(response.versions.iter().map(|v| v.id.clone()).collect())
        Ok(response.versions)
    }
}
