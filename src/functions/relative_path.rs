use crate::prelude::Result;
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

pub fn relative_path(file_name: &str) -> Result<PathBuf> {
    let current_dir = current_dir()?;
    let file_path = current_dir.join(Path::new(file_name));

    Ok(file_path)
}
