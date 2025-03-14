use super::Loader;

pub struct Neoforge;

impl Loader for Neoforge {
    fn get_versions() -> Vec<String> {
        vec!["1.16.5".to_string(), "1.17.1".to_string()]
    }

    async fn download(&self, version: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
}
