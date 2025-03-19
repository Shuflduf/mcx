use std::{fs::{self, File}, io::Write};

pub async fn add(token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let download_url = if token.starts_with("https://") {
        token
    } else {
        println!("Invalid URL: {}", token);
        return Ok(());
    };

    println!("Downloading mod from {}", download_url);

    let data = reqwest::get(download_url).await?.bytes().await?;
    if fs::create_dir("mods").is_err() {
        println!("Mods directory already exists, skipping creation.");
    }
    let mut file = File::create(
        format!(
            "mods/{}",
            token.rsplit_once('/').unwrap().1
        )
    )?;
    file.write_all(&data)?;
    println!("Mod downloaded succesfully");

    Ok(())
}

pub async fn list() -> Result<(), Box<dyn std::error::Error>> {
    let mods = fs::read_dir("mods")?;
    for mod_file in mods {
        let mod_file = mod_file?;
        println!("> {}", mod_file.file_name().into_string().unwrap());
    }

    Ok(())
}
