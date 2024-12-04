use std::process::Command;

use chrono_tz::Tz;

use crate::functions::run_commands::run_command;

use crate::prelude::Result;

pub fn set_timezone(region: &str, city: &str) -> Result<()> {
    format!("{}/{}", region, city).parse::<Tz>()?;

    run_command(
        Command::new("ln")
            .arg("-sf")
            .arg(format!("/usr/share/zoneinfo/{}/{}", region, city))
            .arg("/etc/localtime"),
    )?;

    Ok(())
}
