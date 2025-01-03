use std::{env::var, fs::File};

use crate::{
    prelude::Result,
    structure_config::config_path::{LOG_CONFIGURATION, ROOT_PATH},
};
use log::info;
use simplelog::*;

pub fn initialize_logger() -> Result<()> {
    let configuration_log = format!("{}/{}", var(ROOT_PATH)?, LOG_CONFIGURATION);
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
            File::create(&configuration_log)?,
        ),
    ])?;
    info!(
        "Logger initialized successfully \nSee logs in {}",
        configuration_log
    );

    Ok(())
}
