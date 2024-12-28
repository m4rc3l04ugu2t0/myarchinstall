use std::marker::PhantomData;

use crate::{
    install_packages::{
        configure_bootloader::configure_bootloader, install_essentials::install_assentials,
    },
    prelude::{Result, Safety, Unsafety, W},
};
use log::info;
use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct Packages {
    pub essentials: Vec<String>,
}

#[derive(Deserialize, Default, Debug)]
pub struct PackagesBuilder<D, E> {
    pub essentials: W<E>,
    data: PhantomData<D>,
}

impl<D, E> PackagesBuilder<D, E> {
    pub fn essentials_valid(
        self,
        essentials: &[String],
    ) -> Result<PackagesBuilder<Unsafety, Vec<String>>> {
        install_assentials(essentials)?;
        info!("Installing bootloader...");
        configure_bootloader()?;
        info!("Bootloader installed successfully");
        Ok(PackagesBuilder {
            essentials: W(essentials.to_vec()),
            data: PhantomData,
        })
    }
}

impl PackagesBuilder<Unsafety, String> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<E> PackagesBuilder<Unsafety, E> {
    pub fn seal(self) -> Result<PackagesBuilder<Safety, E>> {
        Ok(PackagesBuilder {
            essentials: self.essentials,
            data: PhantomData,
        })
    }
}

impl PackagesBuilder<Safety, Vec<String>> {
    pub fn build(self) -> Result<Packages> {
        Ok(Packages {
            essentials: self.essentials.0,
        })
    }
}
