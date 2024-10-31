use std::{
    io::Write,
    process::{Command, Stdio},
};

use crate::ConfigureError;

pub fn run_passwd_command(password: &str, user_name: &str) -> Result<(), ConfigureError> {
    let mut child = Command::new("passwd")
        .arg(user_name)
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|err| ConfigureError::RunCommand(err.to_string()))?;

    if let Some(stdin) = &mut child.stdin {
        writeln!(stdin, "{}", password).map_err(|e| {
            ConfigureError::RunCommand(format!(
                "Failed to write password to stdin(first time). Error: {}",
                e
            ))
        })?;
        writeln!(stdin, "{}", password).map_err(|e| {
            ConfigureError::RunCommand(format!(
                "Failed to write password to stdin(second time). Error: {}",
                e
            ))
        })?;
    } else {
        return Err(ConfigureError::RunCommand(
            "Enable to access stdin from `passwd` command.".to_string(),
        ));
    }

    let output = child
        .wait_with_output()
        .map_err(|err| ConfigureError::RunCommand(err.to_string()))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(ConfigureError::RunCommand(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }
}
