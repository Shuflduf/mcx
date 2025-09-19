use color_eyre::eyre::{eyre, Result};

use crate::config::{self, LoaderName};

pub fn download_from_slug(slug: &str) -> Result<()> {
    let version_info = config::get_version_info()?;
    if version_info.name == LoaderName::Vanilla {
        return Err(eyre!("Mods are not supported for Vanilla"));
    }
    Ok(())
}
