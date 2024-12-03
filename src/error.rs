use std::{fmt, path::PathBuf};

#[derive(Debug)]
pub struct Trace {
    pub filename: &'static str,
    pub function: &'static str,
    pub description: String,
}

impl fmt::Display for Trace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "File: {}, Function: {}, Description: {}",
            self.filename, self.function, self.description
        )
    }
}

#[derive(Debug)]
pub enum Error {
    ReadFile {
        source: std::io::Error,
        context: String,
        backtrace: Trace,
    },
    OpenFile {
        source: std::io::Error,
        context: String,
        backtrace: Trace,
    },
    WriteFile {
        source: std::io::Error,
        context: String,
        backtrace: Trace,
    },
    CreateDirOrFile {
        source: std::io::Error,
        context: String,
        backtrace: Trace,
    },
    FromStr {
        source: toml::de::Error,
        context: String,
        backtrace: Trace,
    },
    SaveState {
        source: serde_json::Error,
        context: String,
        backtrace: Trace,
    },
    Timezone {
        source: chrono_tz::ParseError,
        context: String,
        backtrace: Trace,
    },
    Logger {
        source: log::SetLoggerError,
        context: String,
        backtrace: Trace,
    },
    GetPath {
        source: PathBuf,
        context: String,
        backtrace: Trace,
    },
    UserNotFound {
        source: String,
        context: String,
        backtrace: Trace,
    },
    CommandExecution {
        source: String,
        context: String,
        backtrace: Trace,
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
                    "Failed to create directory or file : {}: {} \nTrace: {}",
                    context, source, backtrace
                )
            }
            Self::ReadFile {
                source,
                context,
                backtrace,
            } => write!(
                f,
                "Failed to read file : {}: {}: \nTrace: {}",
                context, source, backtrace
            ),
            Self::OpenFile {
                source,
                context,
                backtrace,
            } => write!(
                f,
                "Failed to open file : {}: {}: \nTrace: {}",
                context, source, backtrace
            ),
            Self::WriteFile {
                source,
                context,
                backtrace,
            } => {
                write!(
                    f,
                    "Failed to write file : {}: {}: \nTrace: {}",
                    context, source, backtrace
                )
            }
            Self::FromStr {
                source,
                context,
                backtrace,
            } => write!(
                f,
                "Failed to parse string : {}: {}: \nTrace: {}",
                context, source, backtrace
            ),
            Self::SaveState {
                source,
                context,
                backtrace,
            } => write!(
                f,
                "Failed to save state : {}: {}: \nTrace: {}",
                context, source, backtrace
            ),
            Self::Timezone {
                source,
                context,
                backtrace,
            } => {
                write!(
                    f,
                    "Failed to parse timezone : {}: {}: \nTrace: {}",
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
                    "Failed to initialize logger : {}: {} \nTrace: {}",
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
                    "Failed to get path : {}: {:?}: \nTrace: {}",
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
                    "User not found : {}: {}: \nTrace: {}",
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
                    "Failed to execute command : {}: {}: \nTrace: {}",
                    context, source, backtrace
                )
            }
        }
    }
}

impl std::error::Error for Error {}
