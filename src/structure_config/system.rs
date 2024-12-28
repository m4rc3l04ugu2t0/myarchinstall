use std::marker::PhantomData;

use log::info;
use serde::Deserialize;

use crate::configure_hostname::set_hostname::set_hostname;
use crate::configure_new_user::set_new_user::set_new_user;
use crate::configure_root::set_root::set_root;
use crate::prelude::{Result, Safety, Unsafety, W};

#[derive(Deserialize, Default, Debug)]
pub struct System {
    pub hostname: String,
    pub root_password: String,
    pub username: String,
    pub user_password: String,
}

#[derive(Deserialize, Default, Debug)]
pub struct SystemBuilder<T, H, R, U, P> {
    pub hostname: W<H>,
    pub root_password: W<R>,
    pub username: W<U>,
    pub user_password: W<P>,
    data: PhantomData<T>,
}

impl<H, R, U, P> SystemBuilder<Unsafety, H, R, U, P> {
    pub fn seal(self) -> Result<SystemBuilder<Safety, H, R, U, P>> {
        Ok(SystemBuilder {
            hostname: self.hostname,
            root_password: self.root_password,
            username: self.username,
            user_password: self.user_password,
            data: PhantomData,
        })
    }
}

impl<T, H, R, U, P> SystemBuilder<T, H, R, U, P> {
    pub fn setup_hostname(
        self,
        hostname: &str,
    ) -> Result<SystemBuilder<Unsafety, String, R, U, P>> {
        info!("Configuring hostname...");
        set_hostname(hostname)?;
        info!("Hostname configured successfully");
        Ok(SystemBuilder {
            hostname: W(hostname.into()),
            root_password: self.root_password,
            username: self.username,
            user_password: self.user_password,
            data: PhantomData,
        })
    }

    pub fn setup_root(self, password: &str) -> Result<SystemBuilder<Unsafety, H, String, U, P>> {
        info!("Configuring root password...");
        set_root(password)?;
        info!("Root password configured successfully");
        Ok(SystemBuilder {
            hostname: self.hostname,
            root_password: W(password.into()),
            username: self.username,
            user_password: self.user_password,
            data: PhantomData,
        })
    }

    pub fn setup_user(
        self,
        user: &str,
        password: &str,
    ) -> Result<SystemBuilder<Unsafety, H, R, String, String>> {
        info!("Configuring new user...");
        set_new_user(user, password)?;
        info!("New user configured successfully");
        Ok(SystemBuilder {
            hostname: self.hostname,
            root_password: self.root_password,
            username: W(user.into()),
            user_password: W(password.into()),
            data: PhantomData,
        })
    }
}

impl SystemBuilder<Unsafety, Unsafety, Unsafety, Unsafety, Unsafety> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl SystemBuilder<Safety, String, String, String, String> {
    pub fn build(self) -> Result<System> {
        Ok(System {
            hostname: self.hostname.0,
            root_password: self.root_password.0,
            username: self.username.0,
            user_password: self.user_password.0,
        })
    }
}
