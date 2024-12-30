use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
};

use chrono::Local;

use crate::prelude::{Error, Result, LOG_PATH};

pub fn run_command(command: &mut Command) -> Result<()> {
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("{}commands.log", LOG_PATH))?;

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
        .open(format!("{}stdout.log", LOG_PATH))?;

    let mut stderr_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("{}stderr.log", LOG_PATH))?;

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

#[test]
fn test_run_command() {
    assert!(run_command(Command::new("ls").arg("-la")).is_ok());
}
