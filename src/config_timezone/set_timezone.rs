use std::process::Command;

use chrono_tz::Tz;

use crate::{
    functions::{run_commands::run_command, state::save_state},
    ConfigureError,
};

pub fn set_timezone(timezone: &str) -> Result<(), ConfigureError> {
    println!("Starting to configure the timezone system.");
    println!("Select the  timezone => (ex: America/Sao_Paulo):");

    if !valid_timezone(timezone) {
        return Err(ConfigureError::ConfigTimezone(format!(
            "Invalid timezone: {}",
            timezone
        )));
    }

    run_command(
        Command::new("ln")
            .arg("-sf")
            .arg(format!("/usr/share/zoneinfo/{}", timezone))
            .arg("/etc/localtime"),
    )?;

    println!("The timezone was configured successfully.");
    Ok(())
}

fn valid_timezone(timezone: &str) -> bool {
    timezone.parse::<Tz>().is_ok()
}
