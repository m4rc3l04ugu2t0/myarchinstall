use std::process::Command;

use crate::{
    error::Trace,
    functions::{relative_path::relative_path, run_commands::run_command},
    prelude::{Error, Result},
};
use chrono_tz::Tz;
use tempfile::tempdir;

pub fn set_timezone(region: &str, city: &str) -> Result<()> {
    let tmp = tempdir().unwrap();
    let timezone_path = tmp.path().join("timezone");
    format!("{}/{}", region, city)
        .parse::<Tz>()
        .map_err(|e| Error::Timezone {
            source: e,
            context: format!("Invalid timezone: /usr/share/zoneinfo/{}/{}", region, city),
            backtrace: Trace {
                filename: file!(),
                function: "fn valid_timezone()",
                description: "format!('{}/{}', region, city).parse::<Tz>();".to_string(),
            },
        })?;

    run_command(
        Command::new("ln")
            .arg("-sf")
            .arg(relative_path(format!("src/{}/{}", region, city).as_str())?)
            .arg(timezone_path.clone()),
    )?;
    Ok(())
}

#[cfg(test)]
mod test_timezone {
    use crate::configure_timezone::set_timezone::set_timezone;

    #[test]
    fn test_set_timezone() {
        assert!(set_timezone("America", "Fortaleza").is_ok());
        assert!(set_timezone("error", "error").is_err());
    }
}
