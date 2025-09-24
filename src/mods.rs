use std::{
    fs::{self, File},
    io::Write,
};

use color_eyre::{eyre::Result, owo_colors::OwoColorize};

use crate::{cli::ModSubcommand, config, modrinth};

pub fn handle_command(command: ModSubcommand) -> Result<()> {
    match command {
        ModSubcommand::Add { id } => modrinth::download_from_id(&id, 0)?,
        ModSubcommand::List => list_mods()?,
        _ => todo!(),
    }
    Ok(())
}

pub fn download_mod_jar(url: &str, name: &str) -> Result<()> {
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
