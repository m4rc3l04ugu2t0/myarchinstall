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
                source,
                context,
                backtrace,
            } => {
                write!(
                    f,
                    "Failed to create directory or file : {}: {} \nBacktrace: {}",
                    context, source, backtrace
                )
            }
            Self::ReadFile {
                source,
                context,
                backtrace,
            } => write!(
                f,
                "Failed to read file : {}: {}: \nBacktrace: {}",
                context, source, backtrace
            ),
            Self::OpenFile {
                source,
                context,
                backtrace,
            } => write!(
                f,
                "Failed to open file : {}: {}: \nBacktrace: {}",
                context, source, backtrace
            ),
            Self::WriteFile {
                source,
                context,
                backtrace,
            } => {
                write!(
                    f,
                    "Failed to write file : {}: {}: \nBacktrace: {}",
                    context, source, backtrace
                )
            }
            Self::FromStr {
                source,
                context,
                backtrace,
            } => write!(
                f,
                "Failed to parse string : {}: {}: \nBacktrace: {}",
                context, source, backtrace
            ),
            Self::SaveState {
                source,
                context,
                backtrace,
            } => write!(
                f,
                "Failed to save state : {}: {}: \nBacktrace: {}",
                context, source, backtrace
            ),
            Self::Timezone {
                source,
                context,
                backtrace,
            } => {
                write!(
                    f,
                    "Failed to parse timezone : {}: {}: \nBacktrace: {}",
                    context, source, backtrace
                )
            }
            Self::Logger {
                source,
                context,
                backtrace,
            } => {
                write!(
                    f,
                    "Failed to initialize logger : {}: {} \nBacktrace: {}",
                    context, source, backtrace
                )
            }
            Self::GetPath {
                source,
                context,
                backtrace,
            } => {
                write!(
                    f,
                    "Failed to get path : {}: {:?}: \nBacktrace: {}",
                    context, source, backtrace
                )
            }
            Self::UserNotFound {
                source,
                context,
                backtrace,
            } => {
                write!(
                    f,
                    "User not found : {}: {}: \nBacktrace: {}",
                    context, source, backtrace
                )
            }
            Self::CommandExecution {
                source,
                context,
                backtrace,
            } => {
                write!(
                    f,
                    "Failed to execute command : {}: {}: \nBacktrace: {}",
                    context, source, backtrace
                )
            }
        }
    }
}

impl std::error::Error for Error {}
