use std::{
    fs::{create_dir_all, OpenOptions},
    io::BufReader,
    path::Path,
};

use serde_json::{from_reader, to_writer};

use crate::prelude::*;
use crate::starting_config::State;

pub fn load_state() -> Result<State> {
    let state_file = "src/state.json";
    if let Ok(file) = OpenOptions::new().read(true).open(state_file) {
        let reader = BufReader::new(file);
        let state: State = from_reader(reader)?;
        Ok(state)
    } else {
        Ok(State { step: 0 })
    }
}

pub fn save_state(state: &State) -> Result<()> {
    let state_file = "src/state.json";

    let state_dir = Path::new(state_file).parent().expect("Error dictory");

    if !state_dir.exists() {
        create_dir_all(state_dir)?;
    }

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(state_file)?;

    to_writer(file, state)?;

    Ok(())
}
