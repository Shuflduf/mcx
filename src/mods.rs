use color_eyre::eyre::Result;

use crate::{cli::ModSubcommand, modrinth};

pub fn handle_command(command: Option<ModSubcommand>) -> Result<()> {
    println!("{command:?}");
    if let Some(ModSubcommand::Add { id }) = command {
        if id.starts_with("https://") {
            download_mod_jar(&id, id.rsplit_once("/").unwrap_or(("", "mod.jar")).1)
        } else {
            modrinth::download_from_slug("create-fabric")?;
        }
    }
    Ok(())
}

fn download_mod_jar(url: &str, name: &str) {
    println!("{url}, {name}")
}
