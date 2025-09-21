use color_eyre::eyre::Result;

mod cli;
mod config;
mod init;
mod loaders;
mod modrinth;
mod mods;
mod run;

// #[tokio::main]
fn main() -> Result<()> {
    color_eyre::install()?;
    match cli::parse_arguments()? {
        cli::Command::Init => init::setup_server()?,
        cli::Command::Run => run::start_server()?,
        cli::Command::Mod { command } => mods::handle_command(command)?,
    }
    Ok(())
}
