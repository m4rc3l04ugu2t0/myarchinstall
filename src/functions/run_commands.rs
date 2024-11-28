use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
};

use chrono::Local;

use crate::prelude::*;

use super::relative_path::relative_path;

pub fn run_command(command: &mut Command) -> Result<()> {
    let log_file_path = relative_path("src/functions/commands.log")?;
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_path)?;

    let command_str = format!("{:#?}", command);
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    writeln!(log_file, "[{}] {}", timestamp, command_str)?;

    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Files to save stdout and stderr
    let stdout_path = relative_path("src/functions/stdout.log")?;
    let stderr_path = relative_path("src/functions/stderr.log")?;

    let mut stdout_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(stdout_path)?;

    let mut stderr_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(stderr_path)?;

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            let line = line?;
            println!("{}", line);
            writeln!(stdout_file, "{}", line)?;
        }
    }

    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);

        for line in reader.lines() {
            let line = line?;
            eprint!("{}", line);
            writeln!(stderr_file, "{}", line)?;
        }
    }

    let status = child.wait()?;

    // Log the result of the command
    let result = if status.success() {
        "Success"
    } else {
        "Failed"
    };

    writeln!(log_file, "[{}] Command completed: {}", timestamp, result)?;

    if !status.success() {
        return Err(Error::Static("Command failed"));
    }

    Ok(())
}
