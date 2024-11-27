use crate::{
    install_packages::{
        configure_bootloader::configure_bootloader, install_essentials::install_assentials,
    },
    prelude::*,
};
use log::info;
use serde::Deserialize;

#[derive(Debug, Default)]
pub struct EssentialValid<'a>(&'a [String]);

#[derive(Debug, Default)]
pub struct EssentialNotValid;

#[derive(Deserialize, Default, Debug)]
pub struct Packages {
    pub essentials: Vec<String>,
}

#[derive(Deserialize, Default, Debug)]
pub struct PackagesBuilder<E> {
    pub essentials: E,
}

impl<E> PackagesBuilder<E> {
    pub fn essentials_valid(
        self,
        essentials: &[String],
    ) -> Result<PackagesBuilder<EssentialValid>> {
        install_assentials(essentials)?;
        info!("Installing bootloader...");
        configure_bootloader()?;
        info!("Bootloader installed successfully");
        Ok(PackagesBuilder {
            essentials: EssentialValid(essentials),
        })
    }
}

impl PackagesBuilder<EssentialNotValid> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl PackagesBuilder<EssentialValid<'_>> {
    pub fn build(self) -> Result<Packages> {
        Ok(Packages {
            essentials: self.essentials.0.to_vec(),
        })
    }
}
