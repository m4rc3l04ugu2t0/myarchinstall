use crate::{
    error::Trace,
    prelude::{Error, Result},
};
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

pub fn relative_path(file_name: &str) -> Result<PathBuf> {
    let current_dir = current_dir().map_err(|e| Error::CreateDirOrFile {
        source: e,
        context: "Failed to get current directory".to_string(),
        backtrace: Trace {
            filename: file!(),
            function: "fn relative_path(file_name: &str) -> Result<PathBuf>",
            description: "current_dir()".to_string(),
        },
    })?;
    let file_path = current_dir.join(Path::new(file_name));

    Ok(file_path)
}
