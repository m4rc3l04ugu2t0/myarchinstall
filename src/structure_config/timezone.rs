use std::marker::PhantomData;

use crate::{
    configure_timezone::set_timezone::set_timezone,
    prelude::{Result, Safety, Unsafety},
};
use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct Timezone {
    pub region: String,
    pub city: String,
}

#[derive(Debug, Default)]
pub struct TimezoneBuilder<P> {
    pub region: String,
    pub city: String,
    data: PhantomData<P>,
}

impl<P> TimezoneBuilder<P> {
    pub fn valid_timezone(self, region: &str, city: &str) -> Result<TimezoneBuilder<Safety>> {
        set_timezone(region, city)?;
        Ok(TimezoneBuilder {
            region: region.to_owned(),
            city: city.to_owned(),
            data: PhantomData,
        })
    }
}

impl TimezoneBuilder<Unsafety> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TimezoneBuilder<Safety> {
    pub fn build(self) -> Result<Timezone> {
        Ok(Timezone {
            region: self.region,
            city: self.city,
        })
    }
}
