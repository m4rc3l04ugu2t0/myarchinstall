use std::{
    fs::{create_dir_all, File},
    path::PathBuf,
};

use dirs_next::config_dir;

use crate::{error::Error, prelude::Result};

pub fn create_path_file(subdir: &str) -> Result<PathBuf> {
    let config_dir =
        config_dir().ok_or_else(|| Error::Generic(format!("Failed to get {}", subdir)))?;

    let file_path = config_dir.join("myarchinstall").join(subdir);

    if let Some(parent) = file_path.parent() {
        if !parent.exists() {
            create_dir_all(parent)?;
        }
    }

    File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&file_path)?;

    Ok(file_path)
}

#[test]
fn test_create_path_file() {
    assert_eq!(
        PathBuf::from("/home/NextLevelCode/.config/myarchinstall/log/configuration.log"),
        create_path_file("log/configuration.log").unwrap()
    )
}
