use std::{
    fs::{self, File},
    io::Write,
};

use crate::config;

pub async fn add(token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let download_url = if token.starts_with("https://") {
        token
    } else {
        &get_url_from_token(token).await?
    };

    println!("Downloading mod from {}", download_url);

    let data = reqwest::get(download_url).await?.bytes().await?;
    if fs::create_dir("mods").is_err() {
        println!("Mods directory already exists, skipping creation.");
    }
    let mut file = File::create(format!(
        "mods/{}",
        download_url
            .rsplit_once('/')
            .map(|(_, filename)| filename.to_string())
            .unwrap_or("mod.jar".to_string())
    ))?;
    file.write_all(&data)?;
    println!("Mod downloaded succesfully");

    Ok(())
}

async fn get_url_from_token(token: &str) -> Result<String, Box<dyn std::error::Error>> {
    let server_version = config::get_value(".", "version");

    let search_url = format!(
        "https://api.modrinth.com/v2/project/{}/version?game_versions=[{}]",
        token, server_version
    );

    println!("Searching for mod at {}", search_url);
    let response = reqwest::get(search_url).await?.text().await?;

    let response_json: serde_json::Value = serde_json::from_str(&response)?;

    println!("JSON: {}", response_json);
    let download_url = response_json[0]["files"][0]["url"].as_str().unwrap();

    println!("Found mod at {}", download_url);
    Ok(download_url.to_string())
}

pub async fn list() -> Result<(), Box<dyn std::error::Error>> {
    let mods = fs::read_dir("mods")?;
    for mod_file in mods {
        let mod_file = mod_file?;
        println!("> {}", mod_file.file_name().into_string().unwrap());
    }

    Ok(())
}
