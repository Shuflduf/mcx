use std::{fs::File, io::copy};

use crate::loaders;
use color_eyre::eyre::Result;

pub fn setup_server() -> Result<()> {
    let loader_name = inquire::Select::new(
        "Loader",
        vec!["Vanilla", "Fabric", "Forge", "NeoForge", "Quilt"],
    )
    .prompt()?;
    let mut loader = loaders::from_name(loader_name);
    loader.setup_versions()?;
    loader.download_server_jar()?;
    Ok(())
}

pub fn download_server_file(url: String) -> Result<()> {
    let mut response = reqwest::blocking::get(url)?;
    let mut file = File::create("server.jar")?;
    copy(&mut response, &mut file)?;
    Ok(())
}
