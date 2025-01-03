use std::{
    env::set_var,
    fs::{create_dir_all, File},
    path::{Path, PathBuf},
};

use crate::prelude::Result;
pub const LOG_COMMANDS: &str = "log/commands.log";
pub const LOG_STDOUT: &str = "log/stdout.log";
pub const LOG_STDERR: &str = "log/stderr.log";
pub const LOG_CONFIGURATION: &str = "log/configuration.log";
pub const STATE_PATH: &str = "configs/state.json";
pub const ROOT_PATH: &str = "ROOT_PATH";

pub fn config_paths() -> Result<()> {
    let config_dir = PathBuf::from("src/config/myarchinstall/"); // config_dir().ok_or_else(|| Error::ConfigDirNotFound)?;
    set_var("ROOT_PATH", &config_dir);
    create_files(
        &config_dir,
        vec![
            LOG_COMMANDS,
            LOG_STDOUT,
            LOG_STDERR,
            LOG_CONFIGURATION,
            STATE_PATH,
        ],
    )?;

    Ok(())
}

pub fn create_files(root_path: &PathBuf, relative_path: Vec<&str>) -> Result<()> {
    for r in relative_path.into_iter() {
        let relative_path = Path::new(root_path).join(r);
        if let Some(p) = relative_path.parent() {
            if !p.exists() {
                create_dir_all(&p)?;
            }
        }
        File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&relative_path)?;
    }
    Ok(())
}

#[test]
fn test_config_paths() {
    assert!(config_paths().is_ok());
}
