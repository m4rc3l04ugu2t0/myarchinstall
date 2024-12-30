use std::fs::{self, read_to_string};
use std::{env, fmt};

use log::info;
use serde::{Deserialize, Serialize};
use std::env::var;
use toml::from_str;

use crate::functions::relative_path::relative_path;
use crate::functions::state::{self, change_state, load_state};
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
            info!("The time zone step has already been completed. Change the file state to redo the step");
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
            info!("The location step has already been completed. Change the file state to redo the step");
            return Ok(());
        }
        info!("Configuring location...");
        self.location = LocationBuilder::new()
            .valid_language(&self.location.language)?
            .valid_keymap(&self.location.keymap)?
            .seal()?
            .build()?;
        info!("Location configured successfully");
        self.save_state(state)?;
        Ok(())
    }

    fn setup_system(&mut self, state: &mut State) -> Result<()> {
        if state.step >= 3 {
            info!("The system step has already been completed. Change the file state to redo the step");
            return Ok(());
        }
        info!("Configuring system...");
        self.system = SystemBuilder::new()
            .setup_hostname(&self.system.hostname)?
            .setup_root(&self.system.root_password)?
            .setup_user(&self.system.username, &self.system.user_password)?
            .seal()?
            .build()?;

        info!("System configured successfully");

        self.save_state(state)?;
        Ok(())
    }

    fn setup_packages(&mut self, state: &mut State) -> Result<()> {
        if state.step >= 4 {
            info!("The packages step has already been completed. Change the file state to redo the step");
            return Ok(());
        }
        info!("Installing packages...");
        self.packages = PackagesBuilder::new()
            .essentials_valid(&self.packages.essentials)?
            .seal()?
            .build()?;

        info!("Packages installed successfully");
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
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "configure_timezone" => {
                config.setup_timezone(&mut state)?;
                return Ok(());
            }
            "setup_location" => {
                config.setup_location(&mut state)?;
                return Ok(());
            }
            "setup_system" => {
                config.setup_system(&mut state)?;
                return Ok(());
            }
            "setup_packages" => {
                config.setup_packages(&mut state)?;
                return Ok(());
            }
            "state" => {
                let value = args[2].parse::<u8>()?;
                change_state(&mut state, value)?;
                return Ok(());
            }
            _ => {
                return Err(Error::Generic(format!("Invalid argument: {}", args[1])));
            }
        }
    }

    config.setup_timezone(&mut state)?;
    config.setup_location(&mut state)?;
    config.setup_system(&mut state)?;
    config.setup_packages(&mut state)?;

    let final_config = config.build()?;
    info!("Configuration completed successfully:\n{}", final_config);

    Ok(())
}

fn config() -> Result<ConfigBuilder> {
    fs::create_dir("/etc/lib/myarchinstall")?;
    let path = var("CONFIG_PATH").unwrap_or("/etc/lib/myarchinstall/setup.toml".to_string());
    let path = relative_path(&path)?;

    if path.exists() {
        let config_content = read_to_string(&path)?;
        let config = from_str(&config_content)?;

        Ok(config)
    } else {
        Err(Error::GetPath(path))
    }
}
