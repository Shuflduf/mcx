use std::fs;

use color_eyre::{
    eyre::{eyre, Result},
    owo_colors::OwoColorize,
};

use crate::{config, init, modrinth};

pub fn setup_modpack(id: String) -> Result<()> {
    if fs::exists("mcx.toml")?
        && !inquire::Confirm::new("Existing MCX config found. Overwrite?")
            .with_default(false)
            .prompt()?
    {
        return Err(eyre!("Operation cancelled"));
    }
    let modpack_info = modrinth::get_project_info(&id)?;
    let target_game_version =
        inquire::Select::new("Version", modpack_info.game_versions.iter().rev().collect())
            .prompt()?;
    println!(
        "  {} {} for {} ({:?})",
        "Downloading".bold().green(),
        modpack_info.title,
        target_game_version,
        modpack_info.loaders[0]
    );
    let modpack_version =
        &modrinth::get_project_versions(&id, &modpack_info.loaders[0], target_game_version)?[0];
    let version_info = config::VersionInfo {
        name: modpack_info.loaders[0].clone(),
        game_version: target_game_version.to_string(),
        loader_version: None,
    };
    config::create_config(version_info)?;
    // setup_config()?;
    // download_all_mods(&id)?;
    for dep in &modpack_version.dependencies {
        modrinth::download_from_id(&dep.project_id, 1)?
    }
    println!(
        "  Succesfully downloaded {} for {}",
        modpack_info.title, target_game_version
    );
    init::send_init_message();
    Ok(())
}
