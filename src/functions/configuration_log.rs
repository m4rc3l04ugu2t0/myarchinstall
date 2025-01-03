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
            File::create(format!("{}{}", var(ROOT_PATH).unwrap(), LOG_CONFIGURATION))?,
        ),
    ])?;
    info!(
        "Logger initialized successfully \nSee logs in {:?}",
        LOG_CONFIGURATION
    );

    Ok(())
}

#[cfg(test)]
mod test_mod_initialize_logger {
    use crate::{
        functions::configuration_log::initialize_logger,
        structure_config::config_path::config_paths,
    };

    #[test]
    fn test_initialize_logger() {
        config_paths().unwrap();
        assert!(initialize_logger().is_ok());
    }
}
