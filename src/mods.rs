use std::{
    fs::{self, File},
    io::Write,
};

use color_eyre::eyre::Result;

use crate::{cli::ModSubcommand, modrinth};

pub fn handle_command(command: Option<ModSubcommand>) -> Result<()> {
    println!("{command:?}");
    if let Some(ModSubcommand::Add { id }) = command {
        modrinth::download_from_id(&id, 0)?;
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
