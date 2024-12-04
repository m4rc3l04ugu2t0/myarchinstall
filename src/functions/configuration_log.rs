use crate::{functions::relative_path::relative_path, prelude::Result};
use log::info;
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
            File::create(relative_path("src/logs/configuration.log")?)?,
        ),
    ])?;
    info!("Logger initialized successfully \nSee logs in src/logs/configuration.log");

    Ok(())
}
