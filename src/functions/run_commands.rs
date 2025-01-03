use std::{
    env::var,
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
};

use chrono::Local;

use crate::{
    prelude::{Error, Result},
    structure_config::config_path::{LOG_COMMANDS, LOG_STDERR, LOG_STDOUT, ROOT_PATH},
};

pub fn run_command(command: &mut Command) -> Result<()> {
    let mut log_file = OpenOptions::new().create(true).append(true).open(format!(
        "{}{}",
        var(ROOT_PATH)?,
        LOG_COMMANDS
    ))?;

    let command_str = format!("{:#?}", command);
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    writeln!(log_file, "[{}] {}", timestamp, command_str)?;

    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stdout_file = OpenOptions::new().create(true).append(true).open(format!(
        "{}{}",
        var(ROOT_PATH).unwrap(),
        LOG_STDOUT
    ))?;

    let mut stderr_file = OpenOptions::new().create(true).append(true).open(format!(
        "{}{}",
        var(ROOT_PATH).unwrap(),
        LOG_STDERR
    ))?;

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
