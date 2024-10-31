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
    Hostname(String),
    Bootloader(String),
}

impl fmt::Display for ConfigureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConfigTimezone(err) => write!(f, "Failed to configure timezone: {}", err),
            Self::CurrentDir(err) => write!(f, "Failed to access the current directory: {}", err),
            Self::ReadFile(err) => write!(f, "Failed to read file: {}", err),
            Self::FromStr(err) => write!(f, "TOML deserialization error: {}", err),
            Self::Setup(err) => write!(f, "Setup error: {}", err),
            Self::ConfigureSystem(err) => write!(f, "System configuration error: {}", err),
            Self::SaveState(err) => write!(f, "Failed to save state: {}", err),
            Self::Timezone(err) => write!(f, "Timezone error: {}", err),
            Self::RunCommand(err) => write!(f, "Command execution error: {}", err),
            Self::LocaleGen(err) => write!(f, "Locale generation error: {}", err),
            Self::Hostname(err) => write!(f, "Hostname configuration error: {}", err),
            Self::Bootloader(err) => write!(f, "Bootloader configuration error: {}", err),
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
