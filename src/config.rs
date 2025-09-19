use color_eyre::eyre::Result;
use serde::Deserialize;

#[derive(Deserialize, PartialEq)]
pub enum LoaderName {
    Vanilla,
    Fabric,
    Forge,
    NeoForge,
    Quilt,
}
#[derive(Deserialize)]
pub struct VersionInfo {
    pub name: LoaderName,
    pub game_version: String,
    pub loader_version: Option<String>,
}
#[derive(Deserialize)]
struct ModInfo {
    name: String,
    slug: String,
    version_date: Option<String>,
}
#[derive(Deserialize)]
struct MCXConfig {
    version_info: VersionInfo,
    mods: Vec<ModInfo>,
}

pub fn get_version_info() -> Result<VersionInfo> {
    Ok(VersionInfo {
        name: LoaderName::Fabric,
        game_version: "1.20.1".into(),
        loader_version: Some("0.17.2".into()),
    })
}
