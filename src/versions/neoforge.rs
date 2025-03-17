use super::Loader;

pub struct Neoforge;

impl Loader for Neoforge {
    async fn get_versions() -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let req_data = reqwest::get("https://maven.neoforged.net/api/maven/versions/releases/net/neoforged/neoforge")
            .await?
            .text()
            .await?;
        let json_data: serde_json::Value = serde_json::from_str(&req_data)?;
        let versions = json_data["versions"]
            .as_array()
            .ok_or("versions array not found")?
            .iter()
            .map(|v| String::from("1.") + v.as_str().unwrap())
            .filter(|v| !v.ends_with("-beta"))
            .collect();
        Ok(versions)
    }

    async fn download(&self, version: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
}
