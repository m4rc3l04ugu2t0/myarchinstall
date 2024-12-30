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

use super::relative_path::relative_path;

const STATE_FILE: &str = "/etc/lib/myarchinstall/state.json";

/// Carrega o estado do arquivo JSON, inicializando um estado padrão se não existir.
pub fn load_state() -> Result<State> {
    if let Ok(file) = OpenOptions::new().read(true).open(STATE_FILE) {
        let reader = BufReader::new(&file);

        match from_reader(reader) {
            Ok(state) => {
                info!("State loaded successfully from {}", STATE_FILE);
                Ok(state)
            }
            Err(e) => {
                info!("Failed to load state from {}: {:?}", STATE_FILE, e);
                Err(Error::ReadFile(e.into()))
            }
        }
    } else {
        info!("State file not found, initializing default state.");
        Ok(State { step: 0 })
    }
}

/// Salva o estado no arquivo JSON, criando o diretório, se necessário.
pub fn save_state(state: &State) -> Result<()> {
    let state_path = relative_path(STATE_FILE)?;
    let state_dir = state_path
        .parent()
        .ok_or_else(|| Error::GetPath(state_path.clone()))?;

    if !state_dir.exists() {
        create_dir_all(state_dir)?;
    }

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&state_path)?;

    to_writer(file, state)?;

    info!("State saved successfully to {}", STATE_FILE);
    Ok(())
}

/// Altera o estado e salva as alterações.
pub fn change_state(state: &mut State, value: u8) -> Result<()> {
    state.step = if value > 4 { 4 } else { value };
    save_state(state)?;
    Ok(())
}
