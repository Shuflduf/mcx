use crate::loaders::MCLoader;
use color_eyre::eyre::Result;

#[derive(Default)]
pub struct NeoforgeLoader {
    game_version: String,
}

impl MCLoader for NeoforgeLoader {
    fn setup_versions(&mut self) -> Result<()> {
        let loader_versions = version_getter::get_loader_versions()?;
        let game_versions = get_mc_versions(&loader_versions, false);
        println!("{game_versions:?}");
        Ok(())
    }
}

fn neoforge_version_into_game_version(neoforge_version: &str) -> String {
    neoforge_version.rsplit_once(".").unwrap().0.to_string() + "1."
}

fn filter_loader_versions(loader_versions: &[String], game_version: &String) -> Vec<String> {
    loader_versions
        .iter()
        .filter(|&v| neoforge_version_into_game_version(v) == *game_version)
        .map(|v| v.to_string())
        .collect()
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
        let response: NeoforgeResponse = serde_json::from_str(&raw_response)?;
        Ok(response.versions)
    }
}
