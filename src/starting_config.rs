use std::{fs::read_to_string, process::exit};

use serde::{Deserialize, Serialize};
use toml::from_str;

use crate::{
    config_timezone::set_timezone::set_timezone,
    configure_location::{
        set_keymaps::{self, set_keymaps},
        set_language::set_language,
    },
    functions::{
        relative_path::relative_path,
        state::{load_state, save_state},
    },
    structure_config::structs_opition::{Drives, Location, Packages, System, Timezone, Zran},
    ConfigureError,
};

#[derive(Deserialize, Debug)]
pub struct Config {
    timezone: Timezone,
    location: Location,
    system: System,
    packages: Packages,
    zran: Zran,
    drives: Drives,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct State {
    pub step: u8,
}

pub struct HandlingConfiguration {
    state: State,
    config: Config,
}

impl HandlingConfiguration {
    fn new(state: State, config: Config) -> Self {
        Self { state, config }
    }

    fn config_timezone(self) -> Result<Self, ConfigureError> {
        set_timezone(&format!(
            "{}/{}",
            self.config.timezone.region, self.config.timezone.city
        ))?;
        save_state(&self.state)?;
        Ok(self)
    }

    fn config_location(self) -> Result<Self, ConfigureError> {
        set_language(&self.config.location.language)?;
        save_state(&self.state)?;
        set_keymaps(&self.config.location.keymap)?;
        save_state(&self.state)?;
        Ok(self)
    }
}

pub fn configure() -> Result<(), ConfigureError> {
    let state = load_state()?;

    let config = config().map_err(|e| ConfigureError::Setup(e.to_string()))?;

    HandlingConfiguration::new(state, config)
        .config_timezone()?
        .config_location()?;

    Ok(())
}

fn config() -> Result<Config, ConfigureError> {
    let file_name = "src/configs/setup.toml";
    let path = relative_path(&file_name)?;

    if path.exists() {
        let config_content = read_to_string(&path)?;
        let config: Config = from_str(&config_content)?;

        Ok(config)
    } else {
        Err(ConfigureError::Setup(
            "Failure to file config.toml".to_string(),
        ))
    }
}
