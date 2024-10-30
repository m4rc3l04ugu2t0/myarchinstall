use core::fmt;
use std::{error, io};

pub type Result<T> = core::result::Result<T, ConfigureError>;

#[derive(Debug)]
pub enum ConfigureError {
    ConfigTimezone(String),
    CurrentDir(String),
    ReadFile(io::Error),
    FromStr(toml::de::Error),
    Setup(String),
    ConfigureSystem(String),
    SaveState(String),
    Timezone(String),
    RunCommand(String),
    LocaleGen(String),
}
impl fmt::Display for ConfigureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConfigTimezone(err) => write!(f, "Error to config timezone: {}", err),
            Self::CurrentDir(err) => write!(f, "Error to current dir: {}", err),
            Self::ReadFile(err) => write!(f, "Error to read file: {}", err),
            Self::FromStr(err) => write!(f, "Error toml: {}", err),
            Self::Setup(err) => write!(f, "Error to setup: {}", err),
            Self::ConfigureSystem(err) => write!(f, "Error to configure system: {}", err),
            Self::SaveState(err) => write!(f, "Error to save state: {}", err),
            Self::Timezone(err) => write!(f, "Error to save state: {}", err),
            Self::RunCommand(err) => write!(f, "Error to run command: {}", err),
            Self::LocaleGen(err) => write!(f, "Error to locale-gen: {}", err),
        }
    }
}

impl error::Error for ConfigureError {}

impl From<io::Error> for ConfigureError {
    fn from(err: io::Error) -> Self {
        Self::ReadFile(err)
    }
}

impl From<toml::de::Error> for ConfigureError {
    fn from(err: toml::de::Error) -> Self {
        Self::FromStr(err)
    }
}
