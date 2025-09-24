use color_eyre::eyre::Result;

mod cli;
mod config;
mod init;
mod loaders;
mod modrinth;
mod mods;
mod pack;
mod run;

// #[tokio::main]
fn main() -> Result<()> {
    color_eyre::config::HookBuilder::default()
        .display_env_section(false)
        .display_location_section(false)
        .install()?;
    match cli::parse_arguments()? {
        cli::Command::Init => init::setup_server()?,
        cli::Command::Pack { id } => pack::setup_modpack(id)?,
        cli::Command::Run => run::start_server()?,
        cli::Command::Mod { command } => mods::handle_command(command)?,
    }
    Ok(())
}
