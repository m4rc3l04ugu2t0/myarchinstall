use std::fs::File;

use crate::{functions::create_path::create_path_file, prelude::Result};
use log::info;
use simplelog::*;

pub fn initialize_logger() -> Result<()> {
    let log_path = create_path_file("configuration.log")?;

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
            File::open(&log_path)?,
        ),
    ])?;
    info!(
        "Logger initialized successfully \nSee logs in {:?}",
        log_path
    );

    Ok(())
}
