use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

use crate::prelude::*;

pub fn run_command(command: &mut Command) -> Result<()> {
    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            println!("{}", line?);
        }
    }

    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);

        for line in reader.lines() {
            eprint!("{}", line?);
        }
    }

    let status = child.wait()?;

    if !status.success() {
        return Err(Error::Static("Command failed"));
    }

    Ok(())
}
