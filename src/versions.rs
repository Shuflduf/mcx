use thiserror::Error;

pub mod vanilla;
pub mod neoforge;

pub use vanilla::Vanilla;
pub use neoforge::Neoforge;

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
    async fn get_versions() -> Result<Vec<String>, Box<dyn std::error::Error>> ;

    async fn download(&self, version: &str, path: &str) -> Result<(), Box<dyn std::error::Error>>;

    fn run();
}

pub async fn download_version(version: &str, path: &str, loader: &str) -> Result<(), DownloadError> {
    match loader {
        "Vanilla" => Vanilla.download(version, path)
            .await
            .map_err(|e| DownloadError::InvalidMetadata(e.to_string())),
        "NeoForge" => Neoforge.download(version, path)
            .await
            .map_err(|e| DownloadError::InvalidMetadata(e.to_string())),
        _ => Err(DownloadError::InvalidMetadata(format!("Invalid loader: {}", loader))),
    }
}

pub async fn get_loader_versions(loader: &str) -> Vec<String> {
    match loader {
        "Vanilla" => Vanilla::get_versions().await.unwrap(),
        "NeoForge" => Neoforge::get_versions().await.unwrap(),
        _ => Vec::new(),
    }
}
