use std::{backtrace::Backtrace, fmt, path::PathBuf};

#[derive(Debug)]
pub enum Error {
    ReadFile {
        source: std::io::Error,
        context: String,
        backtrace: Backtrace,
    },
    OpenFile {
        source: std::io::Error,
        context: String,
        backtrace: Backtrace,
    },
    WriteFile {
        source: std::io::Error,
        context: String,
        backtrace: Backtrace,
    },
    CreateDirOrFile {
        source: std::io::Error,
        context: String,
        backtrace: Backtrace,
    },
    FromStr {
        source: toml::de::Error,
        context: String,
        backtrace: Backtrace,
    },
    SaveState {
        source: serde_json::Error,
        context: String,
        backtrace: Backtrace,
    },
    Timezone {
        source: chrono_tz::ParseError,
        context: String,
        backtrace: Backtrace,
    },
    Logger {
        source: log::SetLoggerError,
        context: String,
        backtrace: Backtrace,
    },
    GetPath {
        source: PathBuf,
        context: String,
        backtrace: Backtrace,
    },
    UserNotFound {
        source: String,
        context: String,
        backtrace: Backtrace,
    },
    CommandExecution {
        source: String,
        context: String,
        backtrace: Backtrace,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreateDirOrFile {
                source, context, ..
            } => {
                write!(
                    f,
                    "Failed to create directory or file : {}: {}",
                    context, source
                )
            }
            Self::ReadFile {
                source, context, ..
            } => write!(f, "Failed to read file : {}: {}", context, source),
            Self::OpenFile {
                source, context, ..
            } => write!(f, "Failed to open file : {}: {}", context, source),
            Self::WriteFile {
                source, context, ..
            } => {
                write!(f, "Failed to write file : {}: {}", context, source)
            }
            Self::FromStr {
                source, context, ..
            } => write!(f, "Failed to parse string : {}: {}", context, source),
            Self::SaveState {
                source, context, ..
            } => write!(f, "Failed to save state : {}: {}", context, source),
            Self::Timezone {
                source, context, ..
            } => {
                write!(f, "Failed to parse timezone : {}: {}", context, source)
            }
            Self::Logger {
                source, context, ..
            } => {
                write!(f, "Failed to initialize logger : {}: {}", context, source)
            }
            Self::GetPath {
                source, context, ..
            } => {
                write!(f, "Failed to get path : {}: {:?}", context, source)
            }
            Self::UserNotFound {
                source, context, ..
            } => {
                write!(f, "User not found : {}: {}", context, source)
            }
            Self::CommandExecution {
                source, context, ..
            } => {
                write!(f, "Failed to execute command : {}: {}", context, source)
            }
        }
    }
}

impl std::error::Error for Error {}
