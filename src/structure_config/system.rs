use serde::Deserialize;

use crate::configure_hostname::set_hostname::set_hostname;
use crate::configure_new_user::set_new_user::set_new_user;
use crate::configure_root::set_root::set_root;
use crate::prelude::*;

#[derive(Default, Debug)]
pub struct HostnameValid(String);

#[derive(Default, Debug)]
pub struct HostnameNotValid;

#[derive(Default, Debug)]
pub struct RootPasswordValid(String);

#[derive(Default, Debug)]
pub struct RootPasswrdNotValid;

#[derive(Default, Debug)]
pub struct UserValid(String);

#[derive(Default, Debug)]
pub struct UserNotValid;

#[derive(Default, Debug)]
pub struct UserPasswordValid(String);

#[derive(Default, Debug)]
pub struct UserPasswordNotValid;

#[derive(Deserialize, Default, Debug)]
pub struct System {
    pub hostname: String,
    pub root_password: String,
    pub username: String,
    pub user_password: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct SystemBuilder<H, R, U, P> {
    pub hostname: H,
    pub root_password: R,
    pub username: U,
    pub user_password: P,
}

impl<H, R, U, P> SystemBuilder<H, R, U, P> {
    pub fn setup_hostname(self, hostname: &str) -> Result<SystemBuilder<HostnameValid, R, U, P>> {
        set_hostname(hostname)?;
        Ok(SystemBuilder {
            hostname: HostnameValid(hostname.into()),
            root_password: self.root_password,
            username: self.username,
            user_password: self.user_password,
        })
    }

    pub fn setup_root(self, password: &str) -> Result<SystemBuilder<H, RootPasswordValid, U, P>> {
        set_root(password)?;
        Ok(SystemBuilder {
            hostname: self.hostname,
            root_password: RootPasswordValid(password.into()),
            username: self.username,
            user_password: self.user_password,
        })
    }

    pub fn setup_user(
        self,
        user: &str,
        password: &str,
    ) -> Result<SystemBuilder<H, R, UserValid, UserPasswordValid>> {
        set_new_user(user, password)?;
        Ok(SystemBuilder {
            hostname: self.hostname,
            root_password: self.root_password,
            username: UserValid(user.into()),
            user_password: UserPasswordValid(password.into()),
        })
    }
}

impl SystemBuilder<HostnameNotValid, RootPasswrdNotValid, UserNotValid, UserPasswordNotValid> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl SystemBuilder<HostnameValid, RootPasswordValid, UserValid, UserPasswordValid> {
    pub fn build(self) -> Result<System> {
        Ok(System {
            hostname: self.hostname.0,
            root_password: self.root_password.0,
            username: self.username.0,
            user_password: self.user_password.0,
        })
    }
}
