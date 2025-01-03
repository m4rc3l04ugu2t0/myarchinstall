use std::{
    env::set_var,
    fs::{create_dir_all, File},
    path::Path,
};

use dirs_next::config_dir;

use crate::prelude::{Error, Result};
pub const LOG_COMMANDS: &str = "/myarchinstall/log/commands.log";
pub const LOG_STDOUT: &str = "/myarchinstall/log/stdout.log";
pub const LOG_STDERR: &str = "/myarchinstall/log/stderr.log";
pub const LOG_CONFIGURATION: &str = "/myarchinstall/log/configuration.log";
pub const STATE_PATH: &str = "/myarchinstall/configs/state.json";
pub const ROOT_PATH: &str = "ROOT_PATH";

pub fn config_paths() -> Result<()> {
    let config_dir = config_dir().ok_or_else(|| Error::ConfigDirNotFound)?;

    set_var(ROOT_PATH, &config_dir);
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

pub fn create_files<T, U>(root_path: T, relative_paths: Vec<U>) -> Result<()>
where
    T: AsRef<Path>,
    U: AsRef<Path>,
{
    for r in relative_paths.into_iter() {
        let relative_path = root_path.as_ref().join(r);
        if let Some(parent) = relative_path.parent() {
            if !parent.exists() {
                create_dir_all(parent)?;
            }
        }
        File::options()
            .write(true)
            .create(true)
            .open(&relative_path)?;
    }
    Ok(())
}
