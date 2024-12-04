use crate::{
    error::Trace,
    functions::relative_path::relative_path,
    prelude::{Error, Result},
};
use simplelog::*;
use std::fs::File;

pub fn initialize_logger() -> Result<()> {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create(relative_path("src/logs/configuration.log")?).map_err(|e| {
                Error::CreateDirOrFile {
                    source: e,
                    context: "Failed to create log file".to_string(),
                    backtrace: Trace {
                        filename: file!(),
                        function: "fn initialize_logger() -> Result<()>",
                        description: "File::create(relative_path('src/logs/configuration.log')?)"
                            .to_string(),
                    },
                }
            })?,
        ),
    ])
    .map_err(|e| Error::Logger {
        source: e,
        context: "Failed to initialize logger".to_string(),
        backtrace: Trace {
            filename: file!(),
            function: "fn initialize_logger() -> Result<()>",
            description: "CombinedLogger::init(vec![...])".to_string(),
        },
    })?;
    Ok(())
}
