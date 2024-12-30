use myarchinstall::{
    functions::configuration_log::initialize_logger, structure_config::starting_config::configure,
};

fn main() {
    dotenv::dotenv().ok();
    if let Err(err) = initialize_logger() {
        return eprintln!("Failed to initialize the logger: {}", err);
    }
    if let Err(err) = configure() {
        eprintln!("Failed to configure the system: {}", err);
    } else {
        println!("Program successfullly executed.");
    }
}
