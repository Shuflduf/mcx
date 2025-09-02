use color_eyre::eyre::Result;

use crate::loaders::{fabric::FabricLoader, vanilla::VanillaLoader};
use std::fmt::Debug;

pub mod fabric;
pub mod vanilla;

pub trait MCLoader: Debug {
    fn tmp(&self);
    fn get_mc_versions(&self) -> Result<Vec<String>> {
        todo!()
    }
}

pub fn from_name(name: &str) -> Box<dyn MCLoader> {
    match name {
        "Fabric" => Box::new(FabricLoader {}),
        "Vanilla" => Box::new(VanillaLoader {}),
        _ => todo!(),
    }
}
