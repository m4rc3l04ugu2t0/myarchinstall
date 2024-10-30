mod config_timezone;
mod errors;
mod functions;
mod starting_config;
mod structure_config;

use starting_config::configure;

pub use self::errors::structure_error::{ConfigureError, Result};

fn main() {
    if let Err(err) = configure() {
        eprintln!("Failed to configure the system: {}", err);
    } else {
        println!("System configured successfully.");
    }
}
