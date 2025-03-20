// Last updated by Shuflduf on 2025-03-20 19:14:16 UTC

use std::fs;
use clap::{CommandFactory, Parser, Subcommand};
use inquire::{Select, Text};
use mods::{add, list};
use versions::ServerLoader;

mod versions;
mod mods;
mod run;
mod config;

#[derive(Parser)]
#[command(author = "Shuflduf")]
#[command(version = "1.0")]
#[command(about = "Command Line Interface to create and manage Minecraft servers")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Minecraft server
    Init,
    /// Run the Minecraft server
    Run,
    /// Manage mods for your Minecraft server
    Mod {
        #[command(subcommand)]
        command: Option<ModSubcommand>,
    },
}

#[derive(Subcommand, Clone)]
enum ModSubcommand {
    /// Add a new mod to the server
    Add {
        /// The token or identifier of the mod to add.
        /// This can be a URL, or if the mod is on Modrinth, the mod ID (e.g. "fabric-api")
        token: String,
    },
    /// List all installed mods
    List,
}

async fn init_server() -> Result<(), Box<dyn std::error::Error>> {
    let loaders = vec!["Vanilla", "NeoForge"];
    let name = Text::new("Server Name: ").prompt()?;
    let loader = Select::new("Loader: ", loaders)
        .prompt()?;

    // Create the server loader
    let server_loader = ServerLoader::from_str(loader)?;

    // Get versions for the selected loader
    let versions = server_loader.get_versions().await?;
    let version = Select::new("Minecraft Version: ", versions)
        .prompt()?;

    if let Err(e) = fs::create_dir(&name) {
        println!("Error creating directory: {}", e);
    }

    println!("Downloading server version...");
    server_loader.download(&version, &name).await?;
    
    println!("Creating MCX configuration");
    config::init(&name, &server_loader.mc_version(&version)?, loader, &version);

    println!("To run your server, run the following commands:");
    println!("\x1b[1;32m $ cd {}/ \x1b[0m", name);
    println!("\x1b[1;32m $ mcx run \x1b[0m");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init) => init_server().await?,
        Some(Commands::Run) => run::start_server()?,
        Some(Commands::Mod { command }) => match command {
            Some(ModSubcommand::Add { token }) => add(&token).await?,
            Some(ModSubcommand::List) => list().await?,
            None => {
                // When no subcommand is provided for 'mod', show help
                let mut cmd = Cli::command();
                if let Some(mod_cmd) = cmd.find_subcommand_mut("mod") {
                    mod_cmd.print_help()?;
                }
            }
        },
        None => {
            // When no command is provided at all, show main help
            let mut cmd = Cli::command();
            cmd.print_help()?;
        }
    }

    Ok(())
}