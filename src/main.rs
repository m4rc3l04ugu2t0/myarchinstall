use myarchinstall::{
    functions::configuration_log::initialize_logger,
    structure_config::{config_path::config_paths, starting_config::configure},
};

fn main() {
    if let Err(err) = config_paths() {
        return eprintln!("Error to config root path: {}", err);
    }
    if let Err(err) = initialize_logger() {
        return eprintln!("Failed to initialize the logger: {}", err);
    }
    if let Err(err) = configure() {
        eprintln!("Failed to configure the system: {}", err);
    } else {
        println!("System configured successfully.");
    }
}
