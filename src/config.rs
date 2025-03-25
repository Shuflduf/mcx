use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

#[derive(Deserialize, Serialize)]
struct Config {
    server: Server,
    mods: HashMap<String, Mod>,
}

#[derive(Deserialize, Serialize)]
struct Server {
    name: String,
    mc_version: String,
    loader: String,
    loader_version: String,
}

#[derive(Deserialize, Serialize)]
struct Mod {
    name: String,
    online: bool,
    file_path: String,
}

pub fn init(name: &str, mc_version: &str, loader: &str, loader_version: &str) {
    fs::write(
        format!("{}/mcx.toml", name),
        toml::toml! {
            [server]
            name = name
            mc_version = mc_version
            loader = loader
            loader_version = loader_version

            [mods]
        }
        .to_string(),
    )
    .expect("Error writing configuration file");
}

pub fn get_value(path: &str, id: &str) -> String {
    let config =
        fs::read_to_string(format!("{}/mcx.toml", path)).expect("Error reading configuration file");
    let config: toml::Value = toml::from_str(&config).unwrap();
    config["server"][id].to_string()
}

pub fn add_mod(path: &str, id: &str, name: &str, online: bool, file_path: &str) {
    let mut config: Config = toml::from_str(
        &fs::read_to_string(format!("{}/mcx.toml", path))
            .expect("Error reading configuration file"),
    )
    .unwrap();

    config.mods.insert(
        id.to_string(),
        Mod {
            name: name.to_string(),
            online,
            file_path: file_path.to_string(),
        },
    );

    let toml_string = toml::to_string(&config).unwrap();
    fs::write(format!("{}/mcx.toml", path), toml_string).expect("Error writing configuration file");
}

pub fn list_mods() -> Vec<String> {
    let config: Config =
        toml::from_str(&fs::read_to_string("mcx.toml").expect("Error reading configuration file"))
            .unwrap();

    config.mods.values().map(|_mod| _mod.name.clone()).collect()
}
