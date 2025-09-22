use std::{
    fs::{self, File},
    io::{copy, Write},
};

use chrono::{DateTime, Utc};
use color_eyre::eyre::{eyre, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, PartialEq, Serialize)]
pub enum LoaderName {
    Vanilla,
    Fabric,
    Forge,
    NeoForge,
    Quilt,
}
#[derive(Deserialize, Serialize)]
pub struct VersionInfo {
    pub name: LoaderName,
    pub game_version: String,
    pub loader_version: Option<String>,
}
#[derive(Deserialize, Serialize)]
pub struct ModInfo {
    pub name: String,
    pub slug: Option<String>,
    pub version_date: Option<DateTime<Utc>>,
}
#[derive(Deserialize, Serialize)]
struct MCXConfig {
    version_info: VersionInfo,
    mods: Option<Vec<ModInfo>>,
}

pub fn get_version_info() -> Result<VersionInfo> {
    Ok(VersionInfo {
        name: LoaderName::Fabric,
        game_version: "1.20.1".into(),
        loader_version: Some("0.17.2".into()),
    })
}

pub fn add_mod(new_mod: ModInfo) -> Result<()> {
    get_config()?;
    Ok(())
}

pub fn create_config(version_info: VersionInfo) -> Result<()> {
    let new_conf = MCXConfig {
        version_info,
        mods: None,
    };
    let mut file = File::create("mcx.toml")?;
    file.write_all(toml::to_string(&new_conf)?.as_bytes())?;
    Ok(())
}

fn get_config() -> Result<MCXConfig> {
    if !fs::exists("mcx.toml").is_ok_and(|b| b) {
        return Err(eyre!(
            "mcx.toml not found. Create a new server by running `mcx init`."
        ));
    }
    Ok(MCXConfig {
        version_info: get_version_info()?,
        mods: Some(vec![]),
    })
    // let file_data = fs::read_to_string
}
