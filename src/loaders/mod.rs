use color_eyre::eyre::Result;

use crate::loaders::{
    fabric::FabricLoader, forge::ForgeLoader, neoforge::NeoforgeLoader, vanilla::VanillaLoader,
};

pub mod fabric;
pub mod forge;
pub mod neoforge;
pub mod vanilla;

pub trait MCLoader {
    fn setup_versions(&mut self) -> Result<()> {
        todo!()
    }
    fn download_server_jar(&mut self) -> Result<()> {
        todo!()
    }
}

pub fn from_name(name: &str) -> Box<dyn MCLoader> {
    match name {
        "Vanilla" => Box::new(VanillaLoader::default()),
        "Fabric" => Box::new(FabricLoader::default()),
        "Forge" => Box::new(ForgeLoader::default()),
        "NeoForge" => Box::new(NeoforgeLoader::default()),
        _ => todo!(),
    }
}

fn include_snapshots() -> Result<bool> {
    Ok(inquire::Confirm::new("Include Snapshots?")
        .with_default(false)
        .prompt()?)
}
