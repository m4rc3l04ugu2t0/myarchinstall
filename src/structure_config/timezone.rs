use std::marker::PhantomData;

use crate::{
    configure_timezone::set_timezone::set_timezone,
    prelude::{Result, Safety, Unsafety, W},
};
use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct Timezone {
    pub region: String,
    pub city: String,
}

#[derive(Debug, Default)]
pub struct TimezoneBuilder<P, T> {
    pub region: W<T>,
    pub city: W<T>,
    data: PhantomData<P>,
}

impl<P, T> TimezoneBuilder<P, T> {
    pub fn valid_timezone(
        self,
        region: &str,
        city: &str,
    ) -> Result<TimezoneBuilder<Unsafety, String>> {
        set_timezone(region, city)?;
        Ok(TimezoneBuilder {
            region: W(region.to_owned()),
            city: W(city.to_owned()),
            data: PhantomData,
        })
    }
}

impl TimezoneBuilder<Unsafety, Unsafety> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> TimezoneBuilder<Unsafety, T> {
    pub fn seal(self) -> Result<TimezoneBuilder<Safety, T>> {
        Ok(TimezoneBuilder {
            city: self.city,
            region: self.region,
            data: PhantomData,
        })
    }
}

impl TimezoneBuilder<Safety, String> {
    pub fn build(self) -> Result<Timezone> {
        Ok(Timezone {
            region: self.region.0,
            city: self.city.0,
        })
    }
}
