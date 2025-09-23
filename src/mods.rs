use color_eyre::eyre::Result;

use crate::{cli::ModSubcommand, modrinth};

pub fn handle_command(command: Option<ModSubcommand>) -> Result<()> {
    println!("{command:?}");
    if let Some(ModSubcommand::Add { id }) = command {
        modrinth::download_from_slug(&id, 0)?;
    }
    Ok(())
}

pub fn download_mod_jar(url: &str, name: &str) -> Result<()> {
    println!("{url}, {name}");
    Ok(())
}
