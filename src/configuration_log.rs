use crate::prelude::*;
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
            File::create("src/logs/configuration.log")?,
        ),
    ])?;
    Ok(())
}
