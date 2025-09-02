use crate::loaders;
use color_eyre::eyre::Result;

pub fn setup_server() -> Result<()> {
    let loader_name = inquire::Select::new(
        "Loader",
        vec!["Vanilla", "Fabric", "Forge", "NeoForge", "Quilt"],
    )
    .prompt()?;
    let loader = loaders::from_name(loader_name);
    println!("{:?}", loader.get_mc_versions()?);
    Ok(())
}
