use crate::functions::run_password_command::run_passwd_command;
use crate::prelude::*;

pub fn set_root(password: &str) -> Result<()> {
    run_passwd_command(password, "root")?;

    Ok(())
}
