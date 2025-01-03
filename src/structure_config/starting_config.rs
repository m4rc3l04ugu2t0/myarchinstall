use std::fmt;
use std::fs::read_to_string;

use log::info;
use serde::{Deserialize, Serialize};
use std::env::var;
use toml::from_str;

use crate::functions::relative_path::relative_path;
use crate::functions::state::{self, load_state};
use crate::prelude::{Error, Result};
use crate::structure_config::location::{Location, LocationBuilder};
use crate::structure_config::packages::{Packages, PackagesBuilder};
use crate::structure_config::system::{System, SystemBuilder};
use crate::structure_config::timezone::{Timezone, TimezoneBuilder};

#[derive(Deserialize, Debug)]
struct Config {
    timezone: Timezone,
    location: Location,
    system: System,
    packages: Packages,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Timezone: {}/{}",
            self.timezone.region, self.timezone.city
        )?;
        writeln!(f, "Language: {:#?}", self.location.language)?;
        writeln!(f, "Keymap: {}", self.location.keymap)?;
        writeln!(f, "Hostname: {}", self.system.hostname)?;
        writeln!(f, "User: {}", self.system.username)?;
        writeln!(f, "Essentials: {:#?}", self.packages.essentials)?;
        Ok(())
    }
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

#[derive(Deserialize)]
struct ConfigBuilder {
    timezone: Timezone,
    location: Location,
    system: System,
    packages: Packages,
}

impl ConfigBuilder {
    fn setup_timezone(&mut self, state: &mut State) -> Result<()> {
        if state.step >= 1 {
            return Ok(());
        }

        self.timezone = TimezoneBuilder::new()
            .valid_timezone(&self.timezone.region, &self.timezone.city)?
            .seal()?
            .build()?;

        self.save_state(state)?;
        Ok(())
    }

    fn setup_location(&mut self, state: &mut State) -> Result<()> {
        if state.step >= 2 {
            return Ok(());
        }

        self.location = LocationBuilder::new()
            .valid_language(&self.location.language)?
            .valid_keymap(&self.location.keymap)?
            .seal()?
            .build()?;

        self.save_state(state)?;
        Ok(())
    }

    fn setup_system(&mut self, state: &mut State) -> Result<()> {
        if state.step >= 3 {
            return Ok(());
        }

        self.system = SystemBuilder::new()
            .setup_hostname(&self.system.hostname)?
            .setup_root(&self.system.root_password)?
            .setup_user(&self.system.username, &self.system.user_password)?
            .seal()?
            .build()?;

        self.save_state(state)?;
        Ok(())
    }

    fn setup_packages(&mut self, state: &mut State) -> Result<()> {
        if state.step >= 4 {
            return Ok(());
        }

        self.packages = PackagesBuilder::new()
            .essentials_valid(&self.packages.essentials)?
            .seal()?
            .build()?;

        self.save_state(state)?;
        Ok(())
    }

    fn save_state(&self, state: &mut State) -> Result<()> {
        state.incremente_state();
        state::save_state(state)?;
        Ok(())
    }
}

impl ConfigBuilder {
    pub fn new(config: ConfigBuilder) -> Self {
        Self { ..config }
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
    let mut config = ConfigBuilder::new(config()?);

    info!("Configuring timezone...");
    config.setup_timezone(&mut state)?;
    info!("Timezone configured successfully");

    info!("Configuring location...");
    config.setup_location(&mut state)?;
    info!("Location configured successfully");

    info!("Configuring system...");
    config.setup_system(&mut state)?;
    info!("System configured successfully");

    info!("Installing packages...");
    config.setup_packages(&mut state)?;
    info!("Packages installed successfully");

    let final_config = config.build()?;
    info!("Configuration completed successfully:\n{}", final_config);

    Ok(())
}

fn config() -> Result<ConfigBuilder> {
    let path = relative_path(&var("CONFIG_PATH").unwrap_or("src/configs/setup.toml".into()))?;

    if path.exists() {
        let config_content = read_to_string(&path)?;
        let config = from_str(&config_content)?;

        Ok(config)
    } else {
        Err(Error::GetPath(path))
    }
}
