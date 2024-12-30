use crate::prelude::Result;
use log::info;
use simplelog::*;
use std::{
    env::var,
    fs::{self, File},
    path::Path,
};

pub fn initialize_logger() -> Result<()> {
    let log_path = if let Some(v) = var("FILE_LOG").ok() {
        v
    } else {
        fs::create_dir_all("/var/log/myarchinstall_log/")?;
        "/var/log/myarchinstall_log/configuration.log".to_string()
    };

    let log_path = Path::new(&log_path);

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
            File::create(&log_path)?,
        ),
    ])?;
    info!(
        "Logger initialized successfully \nSee logs in {:?}",
        log_path
    );

    Ok(())
}
