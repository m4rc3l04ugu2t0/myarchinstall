use std::{
    fs::{create_dir_all, OpenOptions},
    io::BufReader,
};

use log::info;
use serde_json::{from_reader, to_writer};

use crate::{
    prelude::{Error, Result},
    structure_config::starting_config::State,
};

use super::create_path::create_path_file;

pub fn load_state() -> Result<State> {
    let state_file = create_path_file("state.json")?;
    if let Ok(file) = OpenOptions::new().read(true).open(&state_file) {
        let reader = BufReader::new(&file);

        match from_reader(reader) {
            Ok(state) => {
                info!("State loaded successfully from {:?}", state_file);
                Ok(state)
            }
            Err(e) => {
                info!("Failed to load state from {:?}: {:?}", state_file, e);
                Err(Error::ReadFile(e.into()))
            }
        }
    } else {
        info!("State file not found, initializing default state.");
        Ok(State { step: 0 })
    }
}

pub fn save_state(state: &State) -> Result<()> {
    let state_file = create_path_file("state.json")?;
    let state_dir = state_file
        .parent()
        .ok_or_else(|| Error::GetPath(state_file.clone()))?;

    if !state_dir.exists() {
        create_dir_all(state_dir)?;
    }

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&state_file)?;

    to_writer(file, state)?;

    info!("State saved successfully to {:?}", state_file);
    Ok(())
}

pub fn change_state(state: &mut State, value: u8) -> Result<()> {
    state.step = if value > 4 { 4 } else { value };
    save_state(state)?;
    Ok(())
}
