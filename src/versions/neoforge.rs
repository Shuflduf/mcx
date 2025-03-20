use std::{fs::File, io::Write};

use super::Loader;

pub struct Neoforge;

impl Loader for Neoforge {
    async fn get_versions() -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let req_data = reqwest::get("https://maven.neoforged.net/api/maven/versions/releases/net/neoforged/neoforge")
            .await?
            .text()
            .await?;
        let json_data: serde_json::Value = serde_json::from_str(&req_data)?;
        let versions = json_data["versions"]
            .as_array()
            .ok_or("versions array not found")?
            .iter()
            .map(|v| v.as_str().ok_or("version not found").unwrap().to_string())
            //.map(|v| String::from("1.") + v.as_str().unwrap())
            //.filter(|v| !v.ends_with("-beta"))
            .collect();
        Ok(versions)
    }

    async fn download(&self, version: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "https://maven.neoforged.net/releases/net/neoforged/neoforge/{}/neoforge-{}-installer.jar",
            version,
            version
        );

        println!("Downloading installer from: {}", url);
        let installer_jar = reqwest::get(&url)
            .await?
            .bytes()
            .await?;

        println!("Writing installer to: {}/installer.jar", path);
        let mut file = File::create(format!("{}/installer.jar", path))?;
        file.write_all(&installer_jar)?;
        println!("Installer downloaded successfully");

        println!("Running installer");
        let mut installer = std::process::Command::new("java")
            .arg("-jar")
            .arg(format!("{}/installer.jar", path))
            .arg("--installServer")
            .arg(path)
            .spawn()?;
        installer.wait()?;
        println!("Installer finished");

        println!("Removing installer");
        std::fs::remove_file(format!("{}/installer.jar", path))?;

        println!("Neoforge server installed successfully");
        Ok(())
    }

    fn run() {
        let script = if cfg!(windows) {
            "./run.bat"
        } else {
            "./run.sh"
        };
    
        let _ = std::process::Command::new(script)
            .spawn()
            .expect("Error starting server")
            .wait();
    }
}
