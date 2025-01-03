use std::{env::var, fs::OpenOptions, io::BufReader};

use log::info;
use serde_json::{from_reader, to_writer};

use crate::{
    prelude::{Error, Result},
    structure_config::{
        config_path::{ROOT_PATH, STATE_PATH},
        starting_config::State,
    },
};

pub fn load_state() -> Result<State> {
    let state_path = format!("{}/{}", var(ROOT_PATH)?, STATE_PATH);
    if let Ok(file) = OpenOptions::new().read(true).open(&state_path) {
        let reader = BufReader::new(&file);

        match from_reader(reader) {
            Ok(state) => {
                info!("State loaded successfully from {:?}", state_path);
                Ok(state)
            }
            Err(e) => {
                info!("Failed to load state from {:?}: {:?}", state_path, e);
                if e.is_eof() {
                    info!("State file is empty, initializing default state.");
                    Ok(State { step: 0 })
                } else {
                    Err(Error::ReadFile(e.into()))
                }
            }
        }
    } else {
        info!("State file not found, initializing default state.");
        Ok(State { step: 0 })
    }
}

pub fn save_state(state: &State) -> Result<()> {
    let state_path = format!("{}/{}", var(ROOT_PATH)?, STATE_PATH);
    let file = OpenOptions::new().write(true).open(&state_path)?;

    to_writer(file, state)?;

    info!("State saved successfully to {:?}", state_path);
    Ok(())
}

pub fn change_state(state: &mut State, value: u8) -> Result<()> {
    state.step = if value > 4 { 4 } else { value };
    save_state(state)?;
    Ok(())
}
