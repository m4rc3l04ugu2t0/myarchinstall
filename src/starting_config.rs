#![allow(unused)]
use std::fs::read_to_string;

use serde::{Deserialize, Serialize};
use toml::from_str;

use crate::functions::relative_path::relative_path;
use crate::functions::state::{self, load_state};
use crate::prelude::*;
use crate::structure_config::location::{Location, LocationBuilder};
use crate::structure_config::packages::{Packages, PackagesBuilder};
use crate::structure_config::system::{System, SystemBuilder};
use crate::structure_config::timezone::{Timezone, TimezoneBuilder};

#[derive(Deserialize)]
pub struct Config {
    timezone: Timezone,
    location: Location,
    system: System,
    packages: Packages,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct State {
    pub step: u8,
}

impl State {
    fn incremente_state(&mut self) {
        self.step += 1;
    }
}

#[derive(Deserialize, Default, Debug)]
pub struct ConfigBuilder {
    timezone: Timezone,
    location: Location,
    system: System,
    packages: Packages,
}

impl ConfigBuilder {
    fn setup_timezone(self) -> Result<ConfigBuilder> {
        let timezone = TimezoneBuilder::new()
            .valid_timezone(self.timezone.region, self.timezone.city)?
            .build()?;

        Ok(ConfigBuilder { timezone, ..self })
    }

    fn setup_location(self) -> Result<ConfigBuilder> {
        let location = LocationBuilder::new()
            .valid_language(&self.location.language)?
            .valid_keymap(&self.location.keymap)?
            .build()?;

        Ok(ConfigBuilder { location, ..self })
    }

    fn setup_system(self) -> Result<ConfigBuilder> {
        let system = SystemBuilder::new()
            .setup_hostname(&self.system.hostname)?
            .setup_root(&self.system.root_password)?
            .setup_user(&self.system.user, &self.system.user_password)?
            .build()?;

        Ok(ConfigBuilder { system, ..self })
    }

    fn setup_packages(self) -> Result<ConfigBuilder> {
        let packages = PackagesBuilder::new()
            .essentials_valid(&self.packages.essentials)?
            .build()?;

        Ok(ConfigBuilder { packages, ..self })
    }

    fn save_state(self, state: &mut State) -> Result<Self> {
        state.incremente_state();
        state::save_state(state)?;
        Ok(self)
    }
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ConfigBuilder {
    fn build(self) -> Result<Config> {
        Ok(Config {
            timezone: self.timezone,
            location: self.location,
            system: self.system,
            packages: self.packages,
        })
    }
}

pub fn configure() -> Result<()> {
    let mut state = load_state()?;

    let config = config()?;

    ConfigBuilder::new()
        .setup_timezone()?
        .save_state(&mut state)?
        .setup_location()?
        .save_state(&mut state)?
        .setup_system()?
        .save_state(&mut state)?
        .setup_packages()?
        .save_state(&mut state)?
        .build()?;

    Ok(())
}

fn config() -> Result<ConfigBuilder> {
    let path_file = "src/configs/setup.toml";
    let path = relative_path(path_file)?;

    if path.exists() {
        let config_content = read_to_string(&path)?;
        let config = from_str(&config_content)?;

        Ok(config)
    } else {
        Err(Error::Setup("Failure to file config.toml".to_string()))
    }
}
