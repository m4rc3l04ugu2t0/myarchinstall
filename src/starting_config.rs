use std::fs::read_to_string;

use serde::{Deserialize, Serialize};
use toml::from_str;

use crate::{
    config_timezone::set_timezone::set_timezone,
    configure_hostname::set_hostname::set_hostname,
    configure_location::{set_keymaps::set_keymaps, set_language::set_language},
    configure_new_user::set_new_user::set_new_user,
    configure_root::set_root::set_root,
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

impl State {
    fn incremente_state(&mut self) {
        self.step += 1;
    }
}

pub struct HandlingConfiguration {
    config: Config,
}

impl HandlingConfiguration {
    fn new(config: Config) -> Self {
        Self { config }
    }

    fn steps(&self, state: &mut State) -> Result<(), ConfigureError> {
        let steps: Vec<Box<dyn Fn() -> Result<(), ConfigureError>>> = vec![
            Box::new(|| {
                set_timezone(&format!(
                    "{}/{}",
                    self.config.timezone.region, self.config.timezone.city
                ))
            }),
            Box::new(|| set_language(&self.config.location.language)),
            Box::new(|| set_keymaps(&self.config.location.keymap)),
            Box::new(|| set_hostname(&self.config.system.hostname)),
            Box::new(|| set_root(&self.config.system.root_password)),
            Box::new(|| {
                set_new_user(
                    &self.config.system.username,
                    &self.config.system.username_password,
                )
            }),
        ];

        for step in steps.iter().skip(state.step.into()) {
            match step() {
                Ok(_) => {
                    state.incremente_state();
                    save_state(&state)?;
                }
                Err(err) => {
                    save_state(&state)?;
                    return Err(err);
                }
            }
        }

        Ok(())
    }
}

pub fn configure() -> Result<(), ConfigureError> {
    let mut state = load_state()?;

    let config = config().map_err(|e| ConfigureError::Setup(e.to_string()))?;
    HandlingConfiguration::new(config).steps(&mut state)?;

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
