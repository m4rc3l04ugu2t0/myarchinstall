use std::marker::PhantomData;

use crate::{
    configure_keymaps::set_keymaps::set_keymaps,
    configure_location::set_language::set_language,
    prelude::{Result, Safety, Unsafety, W},
};
use log::info;
use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct Location {
    pub language: Vec<String>,
    pub keymap: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct LocationBuilder<P, L, K> {
    pub language: W<Vec<L>>,
    pub keymap: W<K>,
    data: PhantomData<P>,
}

impl<P, L, K> LocationBuilder<P, L, K> {
    pub fn valid_language(
        self,
        language: &[String],
    ) -> Result<LocationBuilder<Unsafety, String, K>> {
        info!("Configuring language...");
        set_language(language)?;
        info!("Language configured successfully");

        Ok(LocationBuilder {
            language: W(language.to_vec()),
            keymap: self.keymap,
            data: PhantomData,
        })
    }

    pub fn valid_keymap(self, keymap: &str) -> Result<LocationBuilder<Unsafety, L, String>> {
        info!("Configuring keymap...");
        set_keymaps(keymap)?;
        info!("Keymap configured successfully");

        Ok(LocationBuilder {
            language: self.language,
            keymap: W(keymap.to_string()),
            data: PhantomData,
        })
    }
}

impl LocationBuilder<Unsafety, Unsafety, Unsafety> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<L, K> LocationBuilder<Unsafety, L, K> {
    pub fn seal(self) -> Result<LocationBuilder<Safety, L, K>> {
        Ok(LocationBuilder {
            keymap: self.keymap,
            language: self.language,
            data: PhantomData,
        })
    }
}

impl LocationBuilder<Safety, String, String> {
    pub fn build(self) -> Result<Location> {
        Ok(Location {
            language: self.language.0,
            keymap: self.keymap.0,
        })
    }
}
