use crate::{
    config::{self, VersionInfo},
    init,
    loaders::MCLoader,
};
use color_eyre::eyre::Result;

#[derive(Default)]
pub struct NeoforgeLoader {
    game_version: String,
    loader_version: String,
}

impl MCLoader for NeoforgeLoader {
    fn setup_versions(&mut self) -> Result<()> {
        let loader_versions = version_getter::get_loader_versions()?;
        let game_versions = get_mc_versions(&loader_versions, false);
        self.game_version = inquire::Select::new("Version", game_versions.clone()).prompt()?;
        self.loader_version = inquire::Select::new(
            "Loader Version",
            filter_loader_versions(&loader_versions, &self.game_version)?,
        )
        .prompt()?;
        config::create_config(VersionInfo {
            name: config::LoaderName::NeoForge,
            game_version: self.game_version.clone(),
            loader_version: Some(self.loader_version.clone()),
        })?;
        println!("{} {}", self.game_version, self.loader_version);
        Ok(())
    }
    fn download_server_jar(&mut self) -> Result<()> {
        let jar_url = format!(
            "https://maven.neoforged.net/releases/net/neoforged/neoforge/{}/neoforge-{}-installer.jar",
            self.loader_version, self.loader_version
        );
        println!("{jar_url}");
        init::download_server_file(jar_url)?;
        Ok(())
    }
}

fn neoforge_version_into_game_version(neoforge_version: &str) -> String {
    "1.".to_string() + neoforge_version.rsplit_once(".").unwrap().0
}

fn filter_loader_versions(
    loader_versions: &[String],
    game_version: &String,
) -> Result<Vec<String>> {
    let include_betas = inquire::Confirm::new("Include Loader Betas?")
        .with_default(false)
        .prompt()?;
    Ok(loader_versions
        .iter()
        .filter(|v| include_betas || !v.ends_with("-beta"))
        .filter(|&v| neoforge_version_into_game_version(v) == *game_version)
        .map(|v| v.to_string())
        .collect())
}

fn get_mc_versions(loader_versions: &[String], include_betas: bool) -> Vec<String> {
    let mut loader_versions: Vec<String> = loader_versions
        .iter()
        .filter(|v| include_betas || !v.ends_with("-beta"))
        .map(|v| neoforge_version_into_game_version(v))
        .collect();
    loader_versions.dedup();
    loader_versions
}

mod version_getter {
    use color_eyre::eyre::Result;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct NeoforgeResponse {
        versions: Vec<String>,
    }

    pub fn get_loader_versions() -> Result<Vec<String>> {
        let url =
            "https://maven.neoforged.net/api/maven/versions/releases/net%2Fneoforged%2Fneoforge";
        let raw_response = reqwest::blocking::get(url)?.text()?;
        let mut response: NeoforgeResponse = serde_json::from_str(&raw_response)?;
        response.versions.reverse();
        Ok(response.versions)
    }
}
