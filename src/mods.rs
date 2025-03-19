pub async fn add(token: &str) -> Result<(), Box<dyn std::error::Error>> {
    if token.starts_with("https://") {
        println!("Downloading mod from {}", token);
    } else {
        println!("Searching for mod {}", token);
    }
    Ok(())
}

pub async fn list() -> Result<(), Box<dyn std::error::Error>> {
    println!("Listing mods...");
    Ok(())
}
