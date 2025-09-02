use clap::{Parser, Subcommand};
use color_eyre::eyre::{OptionExt, Result};
use strum_macros::Display;

#[derive(Parser)]
#[command(author = "Shuflduf")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Command Line Interface to create and manage Minecraft servers")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Display)]
pub enum Command {
    /// Initialize a new Minecraft server
    Init,
    /// Run the Minecraft server
    Run,
    // Manage mods for your Minecraft server
    // Mod {
    //     #[command(subcommand)]
    //     command: Option<ModSubcommand>,
    // },
}

pub fn parse_arguments() -> Result<Command> {
    Cli::parse().command.ok_or_eyre("Invalid command")
}
