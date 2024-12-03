use std::process::Command;

use crate::{
    error::Trace,
    functions::run_commands::run_command,
    prelude::{Error, Result},
};
use chrono_tz::Tz;
use serde::Deserialize;

#[derive(Default, Debug)]
pub struct RegioValid(String);

#[derive(Default, Debug)]
pub struct RegionNotValid;

#[derive(Default, Debug)]
pub struct CityValid(String);

#[derive(Default, Debug)]
pub struct CityNotValid;

#[derive(Deserialize, Default, Debug)]
pub struct Timezone {
    pub region: String,
    pub city: String,
}

#[derive(Debug, Default)]
pub struct TimezoneBuilder<R, C> {
    pub region: R,
    pub city: C,
}

impl<R, C> TimezoneBuilder<R, C> {
    pub fn valid_timezone(
        self,
        region: &str,
        city: &str,
    ) -> Result<TimezoneBuilder<RegioValid, CityValid>> {
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
                .arg(format!("/usr/share/zoneinfo/{}/{}", region, city))
                .arg("/etc/localtime"),
        )?;

        Ok(TimezoneBuilder {
            region: RegioValid(region.to_owned()),
            city: CityValid(city.to_owned()),
        })
    }
}

impl TimezoneBuilder<RegionNotValid, CityNotValid> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TimezoneBuilder<RegioValid, CityValid> {
    pub fn build(self) -> Result<Timezone> {
        Ok(Timezone {
            region: self.region.0,
            city: self.city.0,
        })
    }
}
