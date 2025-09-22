use color_eyre::eyre::{OptionExt, Result};

use crate::{cli::ModSubcommand, modrinth};

pub fn handle_command(command: Option<ModSubcommand>) -> Result<()> {
    println!("{command:?}");
    if let Some(ModSubcommand::Add { id }) = command {
        // i need better url checking
        if id.starts_with("https://") {
            download_mod_jar(&id, id.rsplit_once("/").ok_or_eyre("Invalid URL")?.1)
        } else {
            modrinth::download_from_slug(&id)?;
        }
    }
    Ok(())
}

fn download_mod_jar(url: &str, name: &str) {
    println!("{url}, {name}")
}
