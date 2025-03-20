use std::{fs::File, future::Future, io::Write, pin::Pin};

use super::{Downloadable, Loader, VersionProvider};

#[derive(Clone)]
pub struct Neoforge;

impl Loader for Neoforge {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let script = if cfg!(windows) {
            "./run.bat"
        } else {
            "./run.sh"
        };

        let _ = std::process::Command::new(script).spawn()?.wait();

        Ok(())
    }
}

impl VersionProvider for Neoforge {
    fn get_versions(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<String>, Box<dyn std::error::Error>>> + Send + '_>>
    {
        Box::pin(async {
            // Your existing implementation
            let req_data = reqwest::get(
                "https://maven.neoforged.net/api/maven/versions/releases/net/neoforged/neoforge",
            )
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
        })
    }

    fn mc_version(&self, loader_version: &str) -> Result<String, Box<dyn std::error::Error>> {
        let no_beta_tag = if let Some(trimmed) = loader_version.strip_suffix("-beta") {
            trimmed
        } else {
            loader_version
        };
        let split: &str = no_beta_tag.rsplit_once(".").unwrap().0;
        if let Some(no_zero) = split.strip_suffix(".0") {
            Ok(format!("1.{}", no_zero))
        } else {
            Ok(format!("1.{}", split))
        }
    }

}

impl Downloadable for Neoforge {
    fn download<'a>(
        &'a self,
        version: &'a str,
        path: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + 'a>> {
        Box::pin(async move {
            let url = format!(
                "https://maven.neoforged.net/releases/net/neoforged/neoforge/{}/neoforge-{}-installer.jar",
                version,
                version
            );

            println!("Downloading installer from: {}", url);
            let installer_jar = reqwest::get(&url).await?.bytes().await?;

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
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // i just wanted to write tests lmao
    #[test]
    fn mc_version_trimming() {
        assert_eq!(Neoforge.mc_version("20.2.17-beta").unwrap(), "1.20.2");
        assert_eq!(Neoforge.mc_version("20.2.17").unwrap(), "1.20.2");
        assert_eq!(Neoforge.mc_version("21.0.60-beta").unwrap(), "1.21");
    }
}