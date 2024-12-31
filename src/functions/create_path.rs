use std::{fs::File, path::PathBuf};

use dirs_next::config_dir;

use crate::{error::Error, prelude::Result};

pub fn create_path_file(path: &str) -> Result<PathBuf> {
    let config_dir =
        config_dir().ok_or_else(|| Error::Generic("Failed to get config dir".to_string()))?;

    let bin_dir = config_dir.join("myarchinstall");
    let path = bin_dir.join(path);
    File::create(&path)?;

    Ok(path)
}
