use std::fs;

use clap::{Parser, Subcommand};
use inquire::{Select, Text};
use reqwest::Client;

mod versions;

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
    println!(
        "Server Name: {}, Version: {}, Loader: {}",
        name, version, loader
    );
    if let Err(e) = fs::create_dir(&name) {
        println!("Error creating directory: {}", e);
    }
    let _ = versions::download_version(&version, &name).await;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;
    
    let response = client
        .get("https://piston-meta.mojang.com/v1/packages/407ceb57bdf113dd320ddab8395901d0aa5dec35/1.21.1.json")
        .send()
        .await?;
    
    let body = response.text().await?;
    println!("{}", body);

    let cli = Cli::parse();
    match &cli.command {
        Commands::Init => init_server().await,
    }


    Ok(())
}
