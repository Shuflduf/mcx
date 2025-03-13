use clap::{Parser, Subcommand};
use inquire::{Select, Text};

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

fn init_server() {
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
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => init_server(),
    }
}
