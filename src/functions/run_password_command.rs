use std::{
    backtrace::Backtrace,
    io::Write,
    process::{Command, Stdio},
};

use log::info;

use crate::prelude::{Error, Result};

pub fn run_passwd_command(password: &str, user_name: &str) -> Result<()> {
    info!("Executing 'passwd' command for user: {}", user_name);
    let user_check =
        Command::new("id")
            .arg(user_name)
            .output()
            .map_err(|e| Error::CommandExecution {
                source: e.to_string(),
                context: format!("Failed to check user: {}", user_name),
                backtrace: Backtrace::capture(),
            })?;

    if !user_check.status.success() {
        return Err(Error::CommandExecution {
            source: "id command failed".to_string(),
            context: "Failed to check user".to_string(),
            backtrace: Backtrace::capture(),
        });
    }

    let mut child = Command::new("passwd")
        .arg(user_name)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| Error::CommandExecution {
            source: e.to_string(),
            context: format!("Failed to run 'passwd' command for user: {}", user_name),
            backtrace: Backtrace::capture(),
        })?;

    if let Some(stdin) = &mut child.stdin {
        writeln!(stdin, "{}", password).map_err(|e| Error::WriteFile {
            source: e,
            context: "Failed to write to stdin".to_string(),
            backtrace: Backtrace::capture(),
        })?;
        writeln!(stdin, "{}", password).map_err(|e| Error::WriteFile {
            source: e,
            context: "Failed to write to stdin".to_string(),
            backtrace: Backtrace::capture(),
        })?;
    }

    let output = child
        .wait_with_output()
        .map_err(|e| Error::CommandExecution {
            source: e.to_string(),
            context: format!("Failed to run 'passwd' command for user: {}", user_name),
            backtrace: Backtrace::capture(),
        })?;

    if output.status.success() {
        Ok(())
    } else {
        Err(Error::CommandExecution {
            source: "run_passwd_command".to_string(),
            context: "Failed to run 'passwd' command".to_string(),
            backtrace: Backtrace::capture(),
        })
    }
}
