use crate::prelude::Result;
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

pub fn relative_path(pathway: &str) -> Result<PathBuf> {
    let current_dir = current_dir()?;
    let file_path = current_dir.join(Path::new(pathway));

    Ok(file_path)
}
