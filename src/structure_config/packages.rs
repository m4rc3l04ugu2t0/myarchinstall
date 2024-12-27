use std::marker::PhantomData;

use crate::{
    install_packages::{
        configure_bootloader::configure_bootloader, install_essentials::install_assentials,
    },
    prelude::{Result, Safety, Unsafety},
};
use log::info;
use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct Packages {
    pub essentials: Vec<String>,
}

#[derive(Deserialize, Default, Debug)]
pub struct PackagesBuilder<E> {
    pub essentials: Vec<String>,
    data: PhantomData<E>,
}

impl<E> PackagesBuilder<E> {
    pub fn essentials_valid(self, essentials: &[String]) -> Result<PackagesBuilder<Safety>> {
        install_assentials(essentials)?;
        info!("Installing bootloader...");
        configure_bootloader()?;
        info!("Bootloader installed successfully");
        Ok(PackagesBuilder {
            essentials: essentials.to_vec(),
            data: PhantomData,
        })
    }
}

impl PackagesBuilder<Unsafety> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl PackagesBuilder<Safety> {
    pub fn build(self) -> Result<Packages> {
        Ok(Packages {
            essentials: self.essentials,
        })
    }
}
