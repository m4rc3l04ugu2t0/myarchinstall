use std::{
    fs::{create_dir_all, File, OpenOptions},
    io::{BufReader, Write},
};

use log::info;
use serde_json::{from_reader, to_writer};

use crate::{
    prelude::{Error, Result},
    structure_config::starting_config::State,
};

use super::relative_path::relative_path;

const STATE_FILE: &str = "/etc/lib/myarchinstall/state.json";

pub fn load_state() -> Result<State> {
    let mut file_state = File::create(STATE_FILE)?;
    file_state.write_all(b"{\"step\": 0}")?;
    if let Ok(mut file) = OpenOptions::new().read(true).open(STATE_FILE) {
        file.write_all(b"{\"step\":0}")?;
        let reader = BufReader::new(&file);

        match from_reader(reader) {
            Ok(state) => {
                info!("State loaded successfully from {:?}", STATE_FILE);
                Ok(state)
            }
            Err(e) => {
                info!("Failed to load state from {:?}: {:?}", STATE_FILE, e);
                Err(Error::ReadFile(e.into()))
            }
        }
    } else {
        info!("State file not found, initializing default state.");
        Ok(State { step: 0 })
    }
}

pub fn save_state(state: &State) -> Result<()> {
    let state_dir = relative_path(STATE_FILE)?;

    if state_dir.exists() {
        create_dir_all(state_dir)?;
    } else {
        return Err(Error::GetPath(state_dir));
    }

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(STATE_FILE)?;

    to_writer(file, state)?;

    Ok(())
}

pub fn change_state(state: &mut State, value: u8) -> Result<()> {
    if value > 4 {
        state.step = 4;
        save_state(state)?;
        return Ok(());
    }

    state.step = value;
    save_state(state)?;
    Ok(())
}
