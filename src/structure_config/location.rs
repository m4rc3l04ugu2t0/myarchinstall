use std::process::Command;

use crate::{
    configure_location::set_language::set_language, functions::run_commands::run_command,
    prelude::*,
};
use log::info;
use serde::Deserialize;

#[derive(Default, Debug)]
pub struct LanguageValid<'a>(&'a [String]);

#[derive(Default, Debug)]
pub struct LanguageNotValid;

#[derive(Default, Debug)]
pub struct KeyMapValid<'a>(&'a str);

#[derive(Default, Debug)]
pub struct KeyMapNotValid;

#[derive(Deserialize, Default, Debug)]
pub struct Location {
    pub language: Vec<String>,
    pub keymap: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct LocationBuilder<L, K> {
    pub language: L,
    pub keymap: K,
}

impl<L, K> LocationBuilder<L, K> {
    pub fn valid_language(self, language: &[String]) -> Result<LocationBuilder<LanguageValid, K>> {
        info!("Configuring language...");
        set_language(language)?;
        info!("Language configured successfully");

        Ok(LocationBuilder {
            language: LanguageValid(language),
            keymap: self.keymap,
        })
    }

    pub fn valid_keymap(self, keymap: &str) -> Result<LocationBuilder<L, KeyMapValid>> {
        info!("Configuring keymap...");
        run_command(
            Command::new("sh")
                .arg("-c")
                .arg(format!("echo KEYMAP={} >> /etc/vconsole.conf", keymap)),
        )?;
        info!("Keymap configured successfully");

        Ok(LocationBuilder {
            language: self.language,
            keymap: KeyMapValid(keymap),
        })
    }
}

impl LocationBuilder<LanguageNotValid, KeyMapNotValid> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl LocationBuilder<LanguageValid<'_>, KeyMapValid<'_>> {
    pub fn build(self) -> Result<Location> {
        Ok(Location {
            language: self.language.0.to_owned(),
            keymap: self.keymap.0.to_owned(),
        })
    }
}
