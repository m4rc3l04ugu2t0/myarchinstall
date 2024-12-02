use std::{
    io::Write,
    process::{Command, Stdio},
};

use log::info;

use crate::prelude::{Error, Result};

pub fn run_passwd_command(password: &str, user_name: &str) -> Result<()> {
    info!("Executing 'passwd' command for user: {}", user_name);
    let user_check = Command::new("id").arg(user_name).output()?;

    if !user_check.status.success() {
        return Err(Error::UserNotFound(user_name.to_string()));
    }

    let mut child = Command::new("passwd")
        .arg(user_name)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(stdin) = &mut child.stdin {
        writeln!(stdin, "{}", password)?;
        writeln!(stdin, "{}", password)?;
    }

    let output = child.wait_with_output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(Error::CommandExecution(
            "Failed to set password".to_string(),
        ))
    }
}
