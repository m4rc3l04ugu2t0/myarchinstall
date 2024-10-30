use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

use crate::ConfigureError;

pub fn relative_path(file_name: &str) -> Result<PathBuf, ConfigureError> {
    let current_dir = current_dir().map_err(|e| ConfigureError::CurrentDir(e.to_string()))?;
    let file_path = current_dir.join(Path::new(file_name));

    Ok(file_path)
}
