use std::{
    fs::{self, File},
    io::Write,
};

use color_eyre::{
    eyre::{eyre, Result},
    owo_colors::OwoColorize,
};

use crate::{cli::ModSubcommand, config, modrinth};

pub fn handle_command(command: ModSubcommand) -> Result<()> {
    match command {
        ModSubcommand::Add { id } => modrinth::download_from_id(&id, 0)?,
        ModSubcommand::List => list_mods()?,
        ModSubcommand::Remove { id } => remove_mod(id)?,
        ModSubcommand::Update => modrinth::update_all_mods()?,
    }
    Ok(())
}

pub fn download_mod_jar(url: &str, name: &str) -> Result<()> {
    println!("TEMP DISABLED");
    return Ok(());
    fs::create_dir_all("mods")?;
    let mut new_mod_file = File::create(format!("mods/{name}.jar"))?;
    new_mod_file.write_all(&reqwest::blocking::get(url)?.bytes()?)?;
    println!("{url}, {name}");
    Ok(())
}

fn list_mods() -> Result<()> {
    let all_installed = config::get_config()?.mods.unwrap_or_default();
    println!(
        "  {} mods currently installed",
        all_installed.len().bold().green()
    );
    for installed in all_installed {
        println!("    {} {}", "-".bold().green(), installed.name)
    }
    Ok(())
}

fn remove_mod(id: String) -> Result<()> {
    // let mod_info = config::get
    if !config::has_mod(&id)? {
        return Err(eyre!("\"{id}\" not found"));
    }
    config::remove_mod(&id)?;
    let mod_path = format!("mods/{id}.jar");
    if fs::exists(&mod_path)? {
        fs::remove_file(&mod_path)?;
    } else {
        return Err(eyre!("{id}.jar not found"));
    }
    Ok(())
}
