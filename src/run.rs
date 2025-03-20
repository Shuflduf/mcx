// Last updated by Shuflduf on 2025-03-20 19:13:08 UTC

use crate::versions::ServerLoader;
use std::fs;

pub fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    if !fs::exists("mcx.toml")? {
        println!("No configuration file found. Run `mcx init` to create a server profile.");
        return Ok(());
    }

    verify_eula();

    let config = fs::read_to_string("mcx.toml")?;
    let config: toml::Value = toml::from_str(&config)?;

    let name = config["server"]["name"]
        .as_str()
        .ok_or("Missing server name in config")?;
    let version = config["server"]["version"]
        .as_str()
        .ok_or("Missing version in config")?;
    let loader_str = config["server"]["loader"]
        .as_str()
        .ok_or("Missing loader in config")?;

    println!("Starting server {} ({} - {})", name, version, loader_str);

    // Create the server loader directly from the string
    let loader = ServerLoader::from_str(loader_str)?;

    // Run the server
    loader.run()?;

    Ok(())
}

fn verify_eula() {
    if !fs::exists("eula.txt").expect("Error checking for EULA file") {
        fs::write("eula.txt", "eula=false").expect("Error writing EULA file");
    }
    let eula = fs::read_to_string("eula.txt").expect("Error reading EULA file");
    if !eula.contains("eula=true") {
        let confirmation = inquire::Confirm::new("Do you accept the Minecraft EULA? (https://account.mojang.com/documents/minecraft_eula)")
            .with_default(false)
            .prompt()
            .unwrap();
        if confirmation {
            fs::write("eula.txt", "eula=true").expect("Error writing EULA file");
        } else {
            println!("EULA not accepted. Exiting.");
            std::process::exit(1);
        }
    }
}
