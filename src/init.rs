use std::{fs::File, io::Write};

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
    send_init_message();
    // println!("JAR DOWNLOADING TEMP DISABLED");
    Ok(())
}

pub fn download_server_file(url: String) -> Result<()> {
    let mut server_file = File::create("server.jar")?;
    server_file.write_all(&reqwest::blocking::get(url)?.bytes()?)?;
    Ok(())
}

pub fn send_init_message() {
    println!("  Enter `mcx run` to start the server!")
}
