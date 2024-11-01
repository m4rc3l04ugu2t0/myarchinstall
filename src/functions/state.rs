use std::{
    fs::{create_dir_all, OpenOptions},
    io::BufReader,
    path::Path,
};

use serde_json::{from_reader, to_writer};

use crate::{starting_config::State, ConfigureError};

pub fn load_state() -> Result<State, ConfigureError> {
    let state_file = "src/state.json";
    if let Ok(file) = OpenOptions::new().read(true).open(state_file) {
        let reader = BufReader::new(file);
        let state: State =
            from_reader(reader).map_err(|e| ConfigureError::SaveState(e.to_string()))?;
        Ok(state)
    } else {
        Ok(State { step: 0 })
    }
}

pub fn save_state(state: &State) -> Result<(), ConfigureError> {
    let state_file = "src/state.json";

    let state_dir = Path::new(state_file).parent().expect("Error dictory");

    if !state_dir.exists() {
        create_dir_all(state_dir).map_err(|e| {
            ConfigureError::SaveState(format!(
                "Failure to create folder {}: {}",
                state_dir.display(),
                e
            ))
        })?;
    }

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(state_file)
        .map_err(|e| ConfigureError::SaveState(format!("Failure to save state: {}", e)))?;
    to_writer(file, state)
        .map_err(|e| ConfigureError::SaveState(format!("Failure to save state: {}", e)))?;

    Ok(())
}
