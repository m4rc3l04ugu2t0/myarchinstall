use std::{marker::PhantomData, process::Command};

use crate::{
    configure_location::set_language::set_language,
    functions::run_commands::run_command,
    prelude::{Result, Safety, Unsafety},
};
use log::info;
use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct Location {
    pub language: Vec<String>,
    pub keymap: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct LocationBuilder<P> {
    pub language: Vec<String>,
    pub keymap: String,
    data: PhantomData<P>,
}

impl<P> LocationBuilder<P> {
    pub fn valid_language(self, language: &[String]) -> Result<LocationBuilder<Safety>> {
        info!("Configuring language...");
        set_language(language)?;
        info!("Language configured successfully");

        Ok(LocationBuilder {
            language: language.to_vec(),
            keymap: self.keymap,
            data: PhantomData,
        })
    }

    pub fn valid_keymap(self, keymap: &str) -> Result<LocationBuilder<Safety>> {
        info!("Configuring keymap...");
        run_command(
            Command::new("sh")
                .arg("-c")
                .arg(format!("echo KEYMAP={} >> /etc/vconsole.conf", keymap)),
        )?;
        info!("Keymap configured successfully");

        Ok(LocationBuilder {
            language: self.language,
            keymap: keymap.to_string(),
            data: PhantomData,
        })
    }
}

impl LocationBuilder<Unsafety> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl LocationBuilder<Safety> {
    pub fn build(self) -> Result<Location> {
        Ok(Location {
            language: self.language,
            keymap: self.keymap,
        })
    }
}
