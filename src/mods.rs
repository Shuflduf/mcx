use color_eyre::eyre::Result;

use crate::{cli::ModSubcommand, modrinth};

pub fn handle_command(command: Option<ModSubcommand>) -> Result<()> {
    println!("{command:?}");
    modrinth::download_from_slug("create-fabric")?;
    Ok(())
}
