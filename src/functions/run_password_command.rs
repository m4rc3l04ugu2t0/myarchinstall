use std::{
    io::Write,
    process::{Command, Stdio},
};

use crate::prelude::*;

pub fn run_passwd_command(password: &str, user_name: &str) -> Result<()> {
    let user_check = Command::new("id").arg(user_name).output()?;

    if !user_check.status.success() {
        return Err(Error::Static("User does not exist"));
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
        Err(Error::Static("Failed to set password"))
    }
}
