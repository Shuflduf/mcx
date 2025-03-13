use inquire::{error::InquireError, Select, Text};

mod versions;

fn main() {
    let name = Text::new("Server Name: ").prompt();
    let version = Select::new("Minecraft Version: ", versions::get_versions()).prompt();
    let loader = Select::new("Loader: ", vec!["Vanilla", "Fabric", "Forge", "NeoForge", "Quilt"]).prompt();
}
