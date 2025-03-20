use std::fs;

pub fn init(name: &str, mc_version: &str, loader: &str, loader_version: &str) {
    fs::write(
        format!("{}/mcx.toml", name),
        format!(
            r#"[server]
name = "{name}"
mc_version = "{mc_version}"
loader = "{loader}"
loader_version = "{loader_version}"
"#,
        ),
    )
    .expect("Error writing configuration file");
}

pub fn get_value(id: &str) -> String {
    let config = fs::read_to_string("mcx.toml").expect("Error reading configuration file");
    let config: toml::Value = toml::from_str(&config).unwrap();
    config["server"][id].to_string()
}
