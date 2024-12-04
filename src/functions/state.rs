use std::{
    fs::{create_dir_all, OpenOptions},
    io::BufReader,
    path::Path,
};

use log::info;
use serde_json::{from_reader, to_writer};

use crate::starting_config::State;
use crate::{
    error::Trace,
    prelude::{Error, Result},
};

use super::relative_path::relative_path;

pub fn load_state() -> Result<State> {
    let state_file = relative_path("src/configs/state.json")?;
    if let Ok(file) = OpenOptions::new().read(true).open(&state_file) {
        let reader = BufReader::new(file);

        match from_reader(reader) {
            Ok(state) => {
                info!("State loaded successfully from {:?}", state_file);
                Ok(state)
            }
            Err(e) => {
                info!("Failed to load state from {:?}: {:?}", state_file, e);
                Err(Box::new(Error::SaveState {
                    source: e,
                    context: "Failed to load state".to_string(),
                    backtrace: Trace {
                        filename: file!(),
                        function: "fn load_state() -> Result<State>",
                        description: "from_reader(reader)".to_string(),
                    },
                }))
            }
        }
    } else {
        info!("State file not found, initializing default state.");
        Ok(State { step: 0 })
    }
}

pub fn save_state(state: &State) -> Result<()> {
    let state_file = relative_path("src/configs/state.json")?;

    let state_dir = Path::new(&state_file)
        .parent()
        .expect("Failed to get parent directory of state file");

    if !state_dir.exists() {
        create_dir_all(state_dir).map_err(|e| Error::CreateDirOrFile {
            source: e,
            context: "Failed to create state directory".to_string(),
            backtrace: Trace {
                filename: file!(),
                function: "fn save_state(state: &State)",
                description: "create_dir_all(state_dir)".to_string(),
            },
        })?;
    }

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(state_file)
        .map_err(|e| Error::OpenFile {
            source: e,
            context: "Failed to open state file".to_string(),
            backtrace: Trace {
                filename: file!(),
                function: "fn save_state(state: &State)",
                description:
                    "OpenOptions::new().write(true).truncate(true).create(true).open(state_file)"
                        .to_string(),
            },
        })?;

    to_writer(file, state).map_err(|e| Error::WriteFile {
        source: e.into(),
        context: "Failed to save state".to_string(),
        backtrace: Trace {
            filename: file!(),
            function: "fn save_state(state: &State)",
            description: "to_writer(file, state)".to_string(),
        },
    })?;

    Ok(())
}
