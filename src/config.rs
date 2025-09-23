use std::{
    fs::{self, File},
    io::Write,
};

use chrono::{DateTime, Utc};
use color_eyre::eyre::{eyre, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum LoaderName {
    Vanilla,
    Fabric,
    Forge,
    NeoForge,
    Quilt,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct VersionInfo {
    pub name: LoaderName,
    pub game_version: String,
    pub loader_version: Option<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ModInfo {
    pub name: String,
    pub id: String,
    pub version_date: DateTime<Utc>,
}
#[derive(Debug, Deserialize, Serialize)]
struct MCXConfig {
    version_info: VersionInfo,
    mods: Option<Vec<ModInfo>>,
}

fn write_config(config: MCXConfig) -> Result<()> {
    let mut file = File::create("mcx.toml")?;
    file.write_all(toml::to_string(&config)?.as_bytes())?;
    Ok(())
}

pub fn has_mod(id: &str) -> Result<bool> {
    Ok(get_config()?
        .mods
        .unwrap_or_default()
        .iter()
        .any(|v| v.id == id))
}

pub fn get_version_info() -> Result<VersionInfo> {
    Ok(get_config()?.version_info)
}

pub fn add_mod(new_mod: ModInfo) -> Result<()> {
    let mut conf = get_config()?;
    conf.mods.get_or_insert_with(Vec::new).push(new_mod);
    write_config(conf)?;
    Ok(())
}

pub fn create_config(version_info: VersionInfo) -> Result<()> {
    let new_conf = MCXConfig {
        version_info,
        mods: None,
    };
    write_config(new_conf)?;
    Ok(())
}

fn get_config() -> Result<MCXConfig> {
    if !fs::exists("mcx.toml").is_ok_and(|b| b) {
        return Err(eyre!(
            "mcx.toml not found. Create a new server by running `mcx init`."
        ));
    }
    let file_contents = fs::read_to_string("mcx.toml")?;
    let conf: MCXConfig = toml::from_str(&file_contents)?;
    // Ok(MCXConfig {
    //     version_info: get_version_info()?,
    //     mods: Some(vec![]),
    // })
    // let file_data = fs::read_to_string
    Ok(conf)
}
