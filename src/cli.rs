use clap::{CommandFactory, Parser, Subcommand};
use color_eyre::eyre::Result;
use strum_macros::Display;

#[derive(Subcommand, Display, Debug)]
#[command(arg_required_else_help = true)]
pub enum ModSubcommand {
    /// Add a mod from Modrinth
    ///
    /// Uses the id of the mod (ex. create-fabric for Create Fabric)
    /// Automatically installs dependencies and prompts for optional dependencies
    #[command(verbatim_doc_comment)]
    Add { id: String },
    /// List downloaded mods
    List,
    /// Update all mods added from Modrinth
    Update,
    /// Remove a mod added from Modrinth
    Remove { id: String },
}
#[derive(Subcommand, Display)]
pub enum Command {
    /// Initialize a new Minecraft server
    Init,
    /// Initialize a new server from a Modrinth modpack id
    Pack { id: String },
    /// Run the Minecraft server
    Run,
    /// Manage mods for your Minecraft server
    Mod {
        #[command(subcommand)]
        command: ModSubcommand,
    },
}

#[derive(Parser)]
#[command(author = "Shuflduf")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Command Line Interface to create and manage Minecraft servers")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

pub fn parse_arguments() -> Result<Command> {
    Cli::parse().command.ok_or_else(|| {
        let _ = Cli::command().print_help();
        std::process::exit(0);
    })
}
