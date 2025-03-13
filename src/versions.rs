use std::fs::read_to_string;

use serde_json::Value;

pub fn get_versions() -> Vec<String> {
    let file_data = read_to_string("src/versions.json").unwrap();
    let json_data: Value = serde_json::from_str(&file_data).unwrap();

    json_data["versions"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|v| v["type"].as_str().unwrap() == "release")
        .map(|v| v["id"].as_str().unwrap().to_string())
        .collect()
}
