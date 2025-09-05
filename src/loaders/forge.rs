use std::collections::HashMap;

use color_eyre::eyre::Result;
use serde::Deserialize;

use crate::loaders::MCLoader;

#[derive(Default)]
pub struct ForgeLoader {
    game_version: String,
    loader_version: String,
}

impl MCLoader for ForgeLoader {
    fn setup_versions(&mut self) -> Result<()> {
        let versions_map = version_getter::get_versions()?;
        let version_keys: Vec<String> = versions_map.keys().cloned().collect();
        let game_versions = remove_tags(version_keys);
        dbg!(game_versions);
        Ok(())
    }
}

fn remove_tags(versions: Vec<String>) -> Vec<String> {
    let mut versions: Vec<String> = versions
        .iter()
        .map(|v| v.rsplit_once("-").unwrap().0.to_string())
        .collect();
    versions.dedup();
    versions
}

mod version_getter {
    use std::{collections::HashMap, fs::File, io::Read};

    use color_eyre::eyre::Result;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct ForgeResponse {
        promos: HashMap<String, String>,
    }

    pub fn get_versions() -> Result<HashMap<String, String>> {
        // let url = "https://files.minecraftforge.net/net/minecraftforge/forge/promotions_slim.json";
        // let raw_response = reqwest::blocking::get(url)?.text()?;
        let raw_response = std::fs::read_to_string("src/responses/forge.json")?;
        let response: ForgeResponse = serde_json::from_str(&raw_response)?;
        Ok(response.promos)
    }
}
