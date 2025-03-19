use std::fs;
use clap::{CommandFactory, Parser, Subcommand};
use inquire::{Select, Text};
use mods::{add, list};

mod versions;
mod mods;
mod run;

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

async fn init_server() {
    let loaders = vec!["Vanilla", "NeoForge"];
    let name = Text::new("Server Name: ").prompt().unwrap();
    let loader = Select::new("Loader: ", loaders)
        .prompt()
        .unwrap();

    let version = Select::new("Minecraft Version: ", versions::get_loader_versions(loader).await)
        .prompt()
        .unwrap();

    if let Err(e) = fs::create_dir(&name) {
        println!("Error creating directory: {}", e);
    }
    if let Err(e) = versions::download_version(&version, &name, loader).await {
        println!("Error downloading version: {}", e);
        return;
    }
    println!("Creating MCLI configuration");
    fs::write(
        format!("{}/mcli.toml", name),
        format!(
            r#"[server]
name = "{name}"
version = "{version}"
loader = "{loader}"
"#,
        ),
    ).expect("Error writing configuration file");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init) => init_server().await,
        Some(Commands::Run) => run::start_server(),
        Some(Commands::Mod { command }) => match command {
            Some(ModSubcommand::Add { token }) => add(&token).await?,
            Some(ModSubcommand::List) => list().await?,
            None => {
                // When no subcommand is provided for 'mod', show help
                let mut cmd = Cli::command();
                if let Some(mod_cmd) = cmd.find_subcommand_mut("mod") {
                    mod_cmd.print_help().unwrap();
                }
            }
        },
        None => {
            // When no command is provided at all, show main help
            let mut cmd = Cli::command();
            cmd.print_help().unwrap();
        }
    }

    Ok(())
}
