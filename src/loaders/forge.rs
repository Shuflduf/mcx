use std::{cmp::Ordering, collections::HashMap};

use color_eyre::eyre::Result;

use crate::{
    config::{self, VersionInfo},
    init,
    loaders::MCLoader,
};

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
        self.game_version = inquire::Select::new("Version", game_versions).prompt()?;
        self.loader_version = inquire::Select::new(
            "Loader Version",
            filter_loader_versions(&versions_map, &self.game_version),
        )
        .prompt()?;
        config::create_config(VersionInfo {
            name: config::LoaderName::Forge,
            game_version: self.game_version.clone(),
            loader_version: Some(self.loader_version.clone()),
        })?;
        Ok(())
    }
    fn download_server_jar(&mut self) -> Result<()> {
        let jar_url = format!(
            "https://maven.minecraftforge.net/net/minecraftforge/forge/{}-{}/forge-{}-{}-installer.jar",
            self.game_version, self.loader_version, self.game_version, self.loader_version
        );
        init::download_server_file(jar_url)?;
        Ok(())
    }
}

fn filter_loader_versions(versions_map: &HashMap<String, String>, version: &String) -> Vec<String> {
    let mut versions: Vec<String> = versions_map
        .iter()
        .filter(|(game_ver, _)| game_ver.starts_with(version))
        .map(|(_, loader_ver)| loader_ver.to_string())
        .collect();
    versions.dedup();
    versions
}

fn remove_tags(versions: Vec<String>) -> Vec<String> {
    let mut versions: Vec<String> = versions
        .iter()
        .map(|v| v.rsplit_once("-").unwrap().0.to_string())
        .filter(|v| !v.contains("_"))
        .collect();
    versions.sort_by(sort_versions);
    versions.dedup();
    versions.reverse();
    versions
}

#[allow(clippy::ptr_arg)]
fn sort_versions(a: &String, b: &String) -> Ordering {
    let a_segments = a.split('.').map(|s| s.parse::<u32>().unwrap_or(0));
    let b_segments = b.split('.').map(|s| s.parse::<u32>().unwrap_or(0));
    for (a_val, b_val) in a_segments.zip(b_segments) {
        match a_val.cmp(&b_val) {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Equal => continue,
        }
    }
    a.len().cmp(&b.len())
}

mod version_getter {
    use std::collections::HashMap;

    use color_eyre::eyre::Result;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct ForgeResponse {
        promos: HashMap<String, String>,
    }

    pub fn get_versions() -> Result<HashMap<String, String>> {
        let url = "https://files.minecraftforge.net/net/minecraftforge/forge/promotions_slim.json";
        let raw_response = reqwest::blocking::get(url)?.text()?;
        let response: ForgeResponse = serde_json::from_str(&raw_response)?;
        Ok(response.promos)
    }
}
