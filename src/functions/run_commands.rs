use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
};

use chrono::Local;

use crate::prelude::{Error, Result};

use super::create_path::create_path_file;

pub fn run_command(command: &mut Command) -> Result<()> {
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(create_path_file("logs/commands.log")?)?;

    let command_str = format!("{:#?}", command);
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    writeln!(log_file, "[{}] {}", timestamp, command_str)?;

    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stdout_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(create_path_file("logs/stdout.log")?)?;

    let mut stderr_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(create_path_file("logs/stderr.log")?)?;

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

    let result = if status.success() {
        "Success"
    } else {
        "Failed"
    };

    writeln!(log_file, "[{}] Command completed: {}", timestamp, result)?;

    if !status.success() {
        return Err(Error::CommandExecution(format!(
            "Command failed: {}",
            command_str
        )));
    }

    Ok(())
}
