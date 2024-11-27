#![allow(unused)]
use crate::prelude::*;
use myarchinstall::starting_config::configure;

mod error;
mod prelude;

fn main() {
    if let Err(err) = configure() {
        eprintln!("Failed to configure the system: {}", err);
    } else {
        println!("System configured successfully.");
    }
}
