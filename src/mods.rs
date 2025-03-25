use std::{
    fs::{self, File},
    io::Write,
};

use crate::config;

pub async fn add(token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (download_url, online) = if token.starts_with("https://") {
        (token.to_string(), false)
    } else {
        (get_url_from_token(token).await?, true)
    };

    println!("Downloading mod from {}", download_url);

    let data = reqwest::get(&download_url).await?.bytes().await?;
    if fs::create_dir("mods").is_err() {
        println!("Mods directory already exists, skipping creation.");
    }

    let filename = download_url
        .rsplit_once('/')
        .map(|(_, filename)| filename.to_string())
        .unwrap_or("mod.jar".to_string());
    let mut file = File::create(format!("mods/{}", filename))?;
    file.write_all(&data)?;
    println!("Mod downloaded succesfully");

    println!("Adding mod to configuration file");
    let mod_name = if online {
        get_mod_name(token).await?
    } else {
        filename.clone()
    };
    config::add_mod(".", token, &mod_name, online, &filename);

    Ok(())
}

async fn get_mod_name(token: &str) -> Result<String, Box<dyn std::error::Error>> {
    let search_url = format!("https://api.modrinth.com/v2/project/{}", token);
    let response = reqwest::get(search_url).await?.text().await?;

    let response_json: serde_json::Value = serde_json::from_str(&response)?;

    Ok(response_json["title"].as_str().unwrap().to_string())
}

async fn get_url_from_token(token: &str) -> Result<String, Box<dyn std::error::Error>> {
    let server_version = config::get_value(".", "mc_version");

    let search_url = format!(
        "https://api.modrinth.com/v2/project/{}/version?game_versions=[{}]&loaders=[{}]",
        token,
        server_version,
        config::get_value(".", "loader").to_lowercase() //"fabric"
    );

    println!("Searching for mod at {}", search_url);
    let response = reqwest::get(search_url).await?.text().await?;

    let response_json: serde_json::Value = serde_json::from_str(&response)?;

    //println!("JSON: {}", response_json);
    let download_url = response_json[0]["files"][0]["url"].as_str().unwrap();

    println!("Found mod at {}", download_url);
    Ok(download_url.to_string())
}

pub async fn list() -> Result<(), Box<dyn std::error::Error>> {
    let mods = config::list_mods();
    if mods.is_empty() {
        println!("No mods found");
    } else {
        println!("Mods:");
        for mod_ in mods {
            println!("> {}", mod_);
        }
    }

    Ok(())
}
