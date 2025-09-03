use color_eyre::eyre::Result;

use crate::loaders::{fabric::FabricLoader, vanilla::VanillaLoader};

pub mod fabric;
pub mod vanilla;

pub trait MCLoader {
    async fn setup_versions(&mut self) -> Result<()> {
        todo!()
    }
    fn download_server_jar(&mut self) -> Result<()> {
        todo!()
    }
}

pub fn from_name(name: &str) -> Box<dyn MCLoader> {
    match name {
        "Fabric" => Box::new(FabricLoader {}),
        "Vanilla" => Box::new(VanillaLoader::default()),
        _ => todo!(),
    }
}
