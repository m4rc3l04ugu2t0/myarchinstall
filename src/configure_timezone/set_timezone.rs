use std::process::Command;

use chrono_tz::Tz;
use tempfile::tempdir;

use crate::functions::run_commands::run_command;

use crate::prelude::Result;

pub fn set_timezone(region: &str, city: &str) -> Result<()> {
    let tmp = tempdir().unwrap();
    let timezone_path = tmp.path().join("zoneinfo");
    let localtime_path = tmp.path().join("localtime");
    format!("{}/{}", region, city).parse::<Tz>()?;

    run_command(
        Command::new("ln")
            .arg("-sf")
            .arg(timezone_path)
            .arg(localtime_path),
    )?;

    Ok(())
}

#[cfg(test)]
mod test_timezone {
    use crate::configure_timezone::set_timezone::set_timezone;

    #[test]
    fn test_set_timezone() {
        assert!(set_timezone("sla", "slass").is_ok());
    }
}
