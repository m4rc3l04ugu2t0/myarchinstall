use std::{env::var, fs::File};

use crate::{
    prelude::Result,
    structure_config::config_path::{LOG_CONFIGURATION, ROOT_PATH},
};
use log::info;
use simplelog::*;

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
            File::create(format!("{}{}", var(ROOT_PATH)?, LOG_CONFIGURATION))?,
        ),
    ])?;
    info!(
        "Logger initialized successfully \nSee logs in {:?}",
        LOG_CONFIGURATION
    );

    Ok(())
}
