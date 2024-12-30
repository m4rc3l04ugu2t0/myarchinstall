use std::{num::ParseIntError, path::PathBuf};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to read file: {0}")]
    ReadFile(#[from] std::io::Error),
    #[error("TOML deserialization error: {0}")]
    FromStr(#[from] toml::de::Error),
    #[error("Failed to save state: {0}")]
    SaveState(#[from] serde_json::Error),
    #[error("Failed to configure timezone: {0}")]
    Timezone(#[from] chrono_tz::ParseError),
    #[error("Logger error: {0}")]
    Logger(#[from] log::SetLoggerError),
    #[error("Failed to get path: {0}")]
    GetPath(PathBuf),
    #[error("User does not exist: {0}")]
    UserNotFound(String),
    #[error("Failed to execute command: {0}")]
    CommandExecution(String),
    #[error("Language not found: {0}")]
    Generic(String),
    #[error("Language not found: {0}")]
    ParseNum(#[from] ParseIntError),
}
