use std::fs::{self, File};
use std::io::Write;

use color_eyre::{
    eyre::{eyre, Result},
    owo_colors::OwoColorize,
};
use toml::map::Map;
use toml::Value;

use crate::config::{self, ModInfo};

pub fn start_server() -> Result<()> {
    let conf = config::get_config()?;
    println!(
        "  {} {:?} {} server {}",
        "Starting".bold().green(),
        conf.version_info.name,
        conf.version_info.game_version,
        if conf.version_info.name != config::LoaderName::Vanilla {
            mod_amount_string(conf.mods.unwrap_or_default())
        } else {
            "".into()
        }
    );
    verify_eula()?;
    std::process::Command::new("java")
        .arg("-jar")
        .arg("server.jar")
        .spawn()
        .expect("Failed to start server.jar")
        .wait()?;
    Ok(())
}

fn verify_eula() -> Result<()> {
    let content = std::fs::read_to_string("eula.txt")?;
    let parsed: toml::Value = toml::from_str(&content)?;
    let eula_accepted = parsed.get("eula").and_then(Value::as_bool).unwrap_or(false);
    if !eula_accepted {
        let accepted = inquire::Confirm::new(
            "Do you accept the Minecraft EULA? (https://aka.ms/MinecraftEULA)",
        )
        .with_default(true)
        .prompt()?;
        if !accepted {
            return Err(eyre!("EULA not accepted"));
        }
        File::create("eula.txt")?.write_all(b"eula=true")?;
    }
    Ok(())
}

fn mod_amount_string(mod_list: Vec<ModInfo>) -> String {
    let mod_count = mod_list.len();
    let s_string = if mod_count != 1 { "s" } else { "" };
    format!("with {mod_count} mod{s_string}")
}
