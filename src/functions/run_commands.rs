use std::{
    backtrace::Backtrace,
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
};

use chrono::Local;

use crate::prelude::{Error, Result};

use super::relative_path::relative_path;

pub fn run_command(command: &mut Command) -> Result<()> {
    let log_file_path = relative_path("src/log/commands.log")?;
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_path)
        .map_err(|e| Error::OpenFile {
            source: e.into(),
            context: "Failed to open log file".to_string(),
            backtrace: Backtrace::capture(),
        })?;

    let command_str = format!("{:#?}", command);
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    writeln!(log_file, "[{}] {}", timestamp, command_str).map_err(|e| Error::WriteFile {
        source: e,
        context: "Failed to write to log file".to_string(),
        backtrace: Backtrace::capture(),
    })?;

    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| Error::CommandExecution {
            source: e.to_string(),
            context: format!("Failed to run command: {}", command_str),
            backtrace: Backtrace::capture(),
        })?;

    // Files to save stdout and stderr
    let stdout_path = relative_path("src/logs/stdout.log")?;
    let stderr_path = relative_path("src/logs/stderr.log")?;

    let mut stdout_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(stdout_path)
        .map_err(|e| Error::OpenFile {
            source: e,
            context: "Failed to open stdout file".to_string(),
            backtrace: Backtrace::capture(),
        })?;

    let mut stderr_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(stderr_path)
        .map_err(|e| Error::OpenFile {
            source: e,
            context: "Failed to open stderr file".to_string(),
            backtrace: Backtrace::capture(),
        })?;

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            let line = line.map_err(|e| Error::ReadFile {
                source: e,
                context: "Failed to read stdout".to_string(),
                backtrace: Backtrace::capture(),
            })?;
            println!("{}", line);
            writeln!(stdout_file, "{}", line).map_err(|e| Error::WriteFile {
                source: e,
                context: "Failed to write to stdout file".to_string(),
                backtrace: Backtrace::capture(),
            })?;
        }
    }

    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);

        for line in reader.lines() {
            let line = line.map_err(|e| Error::ReadFile {
                source: e,
                context: "Failed to read stderr".to_string(),
                backtrace: Backtrace::capture(),
            })?;
            eprint!("{}", line);
            writeln!(stderr_file, "{}", line).map_err(|e| Error::WriteFile {
                source: e,
                context: "Failed to write to stderr file".to_string(),
                backtrace: Backtrace::capture(),
            })?;
        }
    }

    let status = child.wait().map_err(|e| Error::CommandExecution {
        source: e.to_string(),
        context: format!("Failed to wait for command: {}", command_str),
        backtrace: Backtrace::capture(),
    })?;

    // Log the result of the command
    let result = if status.success() {
        "Success"
    } else {
        "Failed"
    };

    writeln!(log_file, "[{}] Command completed: {}", timestamp, result).map_err(|e| {
        Error::WriteFile {
            source: e,
            context: "Failed to write to log file".to_string(),
            backtrace: Backtrace::capture(),
        }
    })?;

    if !status.success() {
        return Err(Error::CommandExecution {
            source: status.to_string(),
            context: format!("Failed to run command: {}", command_str),
            backtrace: Backtrace::capture(),
        });
    }

    Ok(())
}
