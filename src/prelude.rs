use serde::Deserialize;

pub use crate::error::Error;
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Default, Debug)]
pub struct Safety;

#[derive(Default, Debug)]
pub struct Unsafety;

#[derive(Debug, Deserialize, Default)]
pub struct W<T>(pub T);

pub const CONFIGS_PATH: &'static str = "/etc/lib/myarchinstall/";
pub const LOG_PATH: &'static str = "/var/log/myarchinstall_log/";
