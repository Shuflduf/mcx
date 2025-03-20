// Last updated by Shuflduf on 2025-03-20 19:07:00 UTC

use std::future::Future;
use std::pin::Pin;
use thiserror::Error;

pub mod neoforge;
pub mod vanilla;

pub use neoforge::Neoforge;
pub use vanilla::Vanilla;

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Network error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Version {0} not found")]
    VersionNotFound(String),
    #[error("Invalid metadata: {0}")]
    InvalidMetadata(String),
}

pub trait Loader {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait VersionProvider {
    fn get_versions<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<String>, Box<dyn std::error::Error>>> + Send + 'a>>;

    fn mc_version(&self, loader_version: &str) -> Result<String, Box<dyn std::error::Error>> ;
}

pub trait Downloadable {
    fn download<'a>(
        &'a self,
        version: &'a str,
        path: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + 'a>>;
}

pub struct ServerLoader {
    runner: Box<dyn Loader>,
    version_provider: Box<dyn VersionProvider>,
    downloader: Box<dyn Downloadable>,
}

impl ServerLoader {
    pub fn from_str(loader: &str) -> Result<Self, DownloadError> {
        match loader {
            "Vanilla" => {
                let vanilla = Vanilla;
                Ok(Self {
                    runner: Box::new(vanilla.clone()),
                    version_provider: Box::new(vanilla.clone()),
                    downloader: Box::new(vanilla),
                })
            }
            "NeoForge" => {
                let neoforge = Neoforge;
                Ok(Self {
                    runner: Box::new(neoforge.clone()),
                    version_provider: Box::new(neoforge.clone()),
                    downloader: Box::new(neoforge),
                })
            }
            _ => Err(DownloadError::InvalidMetadata(format!(
                "Invalid loader: {}",
                loader
            ))),
        }
    }

    pub fn mc_version(&self, loader_version: &str) -> Result<String, Box<dyn std::error::Error>> {
        self.version_provider.mc_version(loader_version)
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.runner.run()
    }

    pub async fn get_versions(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        self.version_provider.get_versions().await
    }

    pub async fn download(
        &self,
        version: &str,
        path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.downloader.download(version, path).await
    }
}

// pub async fn download_version(
//     version: &str,
//     path: &str,
//     loader: &str,
// ) -> Result<(), DownloadError> {
//     let loader = ServerLoader::from_str(loader)?;
//     loader
//         .download(version, path)
//         .await
//         .map_err(|e| DownloadError::InvalidMetadata(e.to_string()))
// }

// pub async fn get_loader_versions(loader: &str) -> Result<Vec<String>, DownloadError> {
//     let loader = ServerLoader::from_str(loader)?;
//     loader
//         .get_versions()
//         .await
//         .map_err(|e| DownloadError::InvalidMetadata(e.to_string()))
// }
