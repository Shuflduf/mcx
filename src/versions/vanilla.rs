use std::{fs::{read_to_string, File}, io::Write};
use serde_json::Value;
use crate::versions::DownloadError;
use super::Loader;

pub struct Vanilla;

impl Loader for Vanilla {
    fn get_versions() -> Vec<String> {
        let file_data = read_to_string("src/versions.json").unwrap();
        let json_data: Value = serde_json::from_str(&file_data).unwrap();

        json_data["versions"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|v| v["type"].as_str().unwrap() == "release")
            .map(|v| v["id"].as_str().unwrap().to_string())
            .collect()
    }

    async fn download(&self, version: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file_data = read_to_string("src/versions.json")?;
        let json_data: Value = serde_json::from_str(&file_data)?;

        // Find version data
        let version_data = json_data["versions"]
            .as_array()
            .ok_or_else(|| DownloadError::InvalidMetadata("versions array not found".to_string()))?
            .iter()
            .find(|v| v["id"].as_str() == Some(version))
            .ok_or_else(|| DownloadError::VersionNotFound(version.to_string()))?;

        // Get metadata URL
        let metadata_url = version_data["url"]
            .as_str()
            .ok_or_else(|| DownloadError::InvalidMetadata("metadata URL not found".to_string()))?;
        
        
        // Download and parse metadata
        println!("Downloading version info from: {metadata_url}");
        let metadata = reqwest::get(metadata_url)
            .await?
            .text()
            .await?;
        
        let parsed_metadata: Value = serde_json::from_str(&metadata)?;
        
        // Get jar URL and download jar
        let jar_url = parsed_metadata["downloads"]["server"]["url"]
            .as_str()
            .ok_or_else(|| DownloadError::InvalidMetadata("jar URL not found in metadata".to_string()))?;
        
        println!("Downloading server jar from: {jar_url}");
        let jar = reqwest::get(jar_url)
            .await?
            .bytes()
            .await?;

        // Save jar file
        let mut file = File::create(format!("{}/{}.jar", path, version))?;
        file.write_all(&jar)?;

        println!("Successfully downloaded version {}", version);
        Ok(())
    }
}

