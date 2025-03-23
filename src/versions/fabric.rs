use super::{Downloadable, Loader, LoaderFuture, VersionProvider};
use crate::config;
use inquire::Select;
use serde_json::Value;
use std::{fs::File, future::Future, io::Write, pin::Pin};

#[derive(Clone)]
pub struct Fabric;

impl Loader for Fabric {
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

impl VersionProvider for Fabric {
    fn get_versions(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<String>, Box<dyn std::error::Error>>> + Send + '_>>
    {
        Box::pin(async {
            let json_data = versions_json().await?;

            Ok(json_data
                .as_array()
                .unwrap()
                .iter()
                .filter(|v| v["stable"].as_bool().unwrap_or(false))
                .map(|v| v["version"].as_str().unwrap().to_string())
                .collect())
        })
    }

    fn mc_version(&self, loader_version: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(loader_version.to_string())
    }

    fn loader_version(&self) -> LoaderFuture {
        Box::pin(async {
            Some(
                Select::new("Fabric version: ", fabric_versions().await.unwrap())
                    .prompt()
                    .unwrap(),
            )
        })
    }
}
impl Downloadable for Fabric {
    fn download<'a>(
        &'a self,
        version: &'a str,
        path: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + 'a>> {
        Box::pin(async move {
            // Get jar url
            let jar_url = format!(
                "https://meta.fabricmc.net/v2/versions/loader/{}/{}/1.0.3/server/jar",
                version,
                config::get_value(path, "loader_version").replace("\"", "")
            );

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
        &reqwest::get("https://meta.fabricmc.net/v2/versions/game")
            .await?
            .text()
            .await?,
    )?;

    Ok(json_data)
}

async fn fabric_versions() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let json_data: Value = serde_json::from_str(
        &reqwest::get("https://meta.fabricmc.net/v2/versions/loader")
            .await?
            .text()
            .await?,
    )?;

    Ok(json_data
        .as_array()
        .unwrap()
        .iter()
        //.filter(|v| v["stable"].as_bool().unwrap_or(false))
        .map(|v| v["version"].as_str().unwrap().to_string())
        .collect())
}
