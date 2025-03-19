use std::fs;

use crate::versions::{Neoforge, Vanilla};
use crate::versions::Loader;

pub fn start_server() {
    if fs::exists("mcx.toml").expect("Error checking for configuration file") {
        verify_eula();
        let config = fs::read_to_string("mcx.toml").expect("Error reading configuration file");
        let config: toml::Value = toml::from_str(&config).expect("Error parsing configuration file");
        let name = config["server"]["name"].as_str().unwrap();
        let version = config["server"]["version"].as_str().unwrap();
        let loader = config["server"]["loader"].as_str().unwrap();

        println!("Starting server {} ({} - {})", name, version, loader);

        match loader {
            "Vanilla" => {
                Vanilla::run();
            }
            "NeoForge" => {
                Neoforge::run();
            }
            _ => {
                println!("Invalid loader: {}", loader);
            }
        }
        //let _ = Command::new("java")
        //    .arg("-jar")
        //    .arg(format!("{}.jar", version))
        //    .arg("nogui")
        //    .spawn()
        //    .expect("Error starting server")
        //    .wait();
    } else {
        println!("No configuration file found. Run `mcx init` to create a server profile.");
    }
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
