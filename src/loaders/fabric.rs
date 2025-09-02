use crate::loaders::MCLoader;

#[derive(Debug)]
pub struct FabricLoader {}

impl MCLoader for FabricLoader {
    fn tmp(&self) {
        println!("fabric gaming")
    }
}
