use std::fs;

use clap::{Parser, Subcommand};
use inquire::{Select, Text};

mod versions;
mod run;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Run,
}

async fn init_server() {
    let name = Text::new("Server Name: ").prompt().unwrap();
    let version = Select::new("Minecraft Version: ", versions::get_versions())
        .prompt()
        .unwrap();
    let loader = Select::new(
        "Loader: ",
        vec!["Vanilla", "Fabric", "Forge", "NeoForge", "Quilt"],
    )
        .prompt()
        .unwrap();

    if let Err(e) = fs::create_dir(&name) {
        println!("Error creating directory: {}", e);
    }
    if let Err(e) = versions::download_version(&version, &name).await {
        println!("Error downloading version: {}", e);
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
            name = name,
            version = version,
            loader = loader
        ),
    ).expect("Error writing configuration file");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => init_server().await,
        Commands::Run => run::start_server(),
    }

    Ok(())
}
