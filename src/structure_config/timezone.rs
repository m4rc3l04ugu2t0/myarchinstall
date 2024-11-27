use std::process::Command;

use crate::{functions::run_commands::run_command, prelude::*};
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
        if !format!("{}/{}", region, city).parse::<Tz>().is_ok() {
            return Err(Error::ConfigTimezone(
                "Invalid Timezone! Make sure it's correct.",
            ));
        }

        println!("Starting to configure the timezone system.");

        run_command(
            Command::new("ln")
                .arg("-sf")
                .arg(format!("/usr/share/zoneinfo/{}/{}", region, city))
                .arg("/etc/localtime"),
        )?;

        println!("The timezone was configured successfully.");
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
