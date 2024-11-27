#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to configure timezone: {0}")]
    ConfigTimezone(&'static str),
    #[error("Failed to access the current directory: {0}")]
    CurrentDir(String),
    #[error("Failed to read file: {0}")]
    ReadFile(#[from] std::io::Error),
    #[error("TOML deserialization error: {0}")]
    FromStr(#[from] toml::de::Error),
    #[error("Setup error: {0}")]
    Setup(String),
    #[error("System configuration error: {0}")]
    ConfigureSystem(String),
    #[error("Failed to save state: {0}")]
    SaveState(#[from] serde_json::Error),
    #[error("Failed to configure timezone: {0}")]
    Timezone(String),
    #[error("Command executation error: {0}")]
    RunCommand(std::io::Error),
    #[error("Locale generation error: {0}")]
    LocaleGen(String),
    #[error("Hostname configiration error: {0}")]
    Hostname(String),
    #[error("Bootloader configuration error: {0}")]
    Bootloader(String),
    #[error("Error: {0}")]
    Static(&'static str),
}
