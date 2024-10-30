use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

use crate::ConfigureError;

pub fn run_command(command: &mut Command) -> Result<(), ConfigureError> {
    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| ConfigureError::RunCommand(e.to_string()))?;

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            match line {
                Ok(line) => println!("{}", line),
                Err(err) => {
                    return Err(ConfigureError::RunCommand(err.to_string()));
                }
            }
        }
    }

    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);

        for line in reader.lines() {
            match line {
                Ok(line) => eprint!("{}", line),
                Err(err) => return Err(ConfigureError::RunCommand(err.to_string())),
            }
        }
    }

    let status = child
        .wait()
        .map_err(|err| ConfigureError::RunCommand(err.to_string()))?;

    if !status.success() {
        return Err(ConfigureError::RunCommand(status.to_string()));
    }

    Ok(())
}
