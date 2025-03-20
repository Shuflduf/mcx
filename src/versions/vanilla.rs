use super::{Downloadable, Loader, VersionProvider};
use crate::versions::DownloadError;
use serde_json::Value;
use std::{fs::File, future::Future, io::Write, pin::Pin};

#[derive(Clone)]
pub struct Vanilla;

impl Loader for Vanilla {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let _ = std::process::Command::new("java")
            .arg("-jar")
            .arg("server.jar")
            .arg("nogui")
            .spawn()?
            .wait();
        Ok(())
    }
}

impl VersionProvider for Vanilla {
    fn get_versions(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<String>, Box<dyn std::error::Error>>> + Send + '_>>
    {
        Box::pin(async {
            let json_data = versions_json().await?;

            Ok(json_data["versions"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|v| v["type"].as_str().unwrap() == "release")
                .map(|v| v["id"].as_str().unwrap().to_string())
                .collect())
        })
    }

    fn mc_version(&self, loader_version: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(loader_version.to_string())
    }

}

impl Downloadable for Vanilla {
    fn download<'a>(
        &'a self,
        version: &'a str,
        path: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + 'a>> {
        Box::pin(async move {
            let json_data = versions_json().await?;

            // Find version data
            let version_data = json_data["versions"]
                .as_array()
                .ok_or_else(|| {
                    DownloadError::InvalidMetadata("versions array not found".to_string())
                })?
                .iter()
                .find(|v| v["id"].as_str() == Some(version))
                .ok_or_else(|| DownloadError::VersionNotFound(version.to_string()))?;

            // Get metadata URL
            let metadata_url = version_data["url"].as_str().ok_or_else(|| {
                DownloadError::InvalidMetadata("metadata URL not found".to_string())
            })?;

            // Download and parse metadata
            println!("Downloading version info from: {metadata_url}");
            let metadata = reqwest::get(metadata_url).await?.text().await?;

            let parsed_metadata: Value = serde_json::from_str(&metadata)?;

            // Get jar URL and download jar
            let jar_url = parsed_metadata["downloads"]["server"]["url"]
                .as_str()
                .ok_or_else(|| {
                    DownloadError::InvalidMetadata("jar URL not found in metadata".to_string())
                })?;

            println!("Downloading server jar from: {jar_url}");
            let jar = reqwest::get(jar_url).await?.bytes().await?;

            // Save jar file
            let mut file = File::create(format!("{}/server.jar", path))?;
            file.write_all(&jar)?;

            println!("Successfully downloaded version {}", version);
            Ok(())
        })
    }
}

async fn versions_json() -> Result<Value, Box<dyn std::error::Error>> {
    let json_data: Value = serde_json::from_str(
        &reqwest::get("https://launchermeta.mojang.com/mc/game/version_manifest.json")
            .await?
            .text()
            .await?,
    )?;

    Ok(json_data)
}
