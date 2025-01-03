use std::process::Command;

use crate::functions::{run_commands::run_command, run_password_command::run_passwd_command};
use crate::prelude::Result;

pub fn set_new_user(username: &str, password: &str) -> Result<()> {
    run_command(
        Command::new("useradd")
            .arg("-m")
            .arg("-g")
            .arg("users")
            .arg("-G")
            .arg("wheel,video,audio,kvm")
            .arg("-s")
            .arg("/bin/bash")
            .arg(username),
    )?;
    run_passwd_command(password, username)?;
    Ok(())
}
