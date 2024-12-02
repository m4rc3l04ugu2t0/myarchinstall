use myarchinstall::{configuration_log::initialize_logger, starting_config::configure};
mod error;

fn main() {
    if let Err(err) = initialize_logger() {
        return eprintln!("Failed to initialize the logger: {}", err);
    }
    if let Err(err) = configure() {
        eprintln!("Failed to configure the system: {}", err);
    } else {
        println!("System configured successfully.");
    }
}
