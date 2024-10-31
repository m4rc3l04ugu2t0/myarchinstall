mod config_timezone;
mod configure_hostname;
mod configure_location;
mod configure_new_user;
mod configure_root;
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
