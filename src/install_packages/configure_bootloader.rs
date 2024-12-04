use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
};

use crate::{
    error::Trace,
    prelude::{Error, Result},
};

pub fn configure_bootloader() -> Result<()> {
    let path = "/etc/mkinitcpio.conf";
    let file = OpenOptions::new()
        .read(true)
        .open(path)
        .map_err(|e| Error::OpenFile {
            source: e,
            context: "Failed to open /etc/mkinitcpio.conf".to_string(),
            backtrace: Trace {
                filename: file!(),
                function: "fn configure_bootloader() -> Result<()>",
                description: format!(
                    "OpenOptions::new()
                            .read(true)
                            .open({})",
                    path
                ),
            },
        })?;

    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        let mut line = line.map_err(|e| Error::ReadFile {
            source: e,
            context: "Failed to read line from /etc/mkinitcpio.conf".to_string(),
            backtrace: Trace {
                filename: file!(),
                function: "fn configure_bootloader() -> Result<()>",
                description: format!("BufReader::new({}).lines()", path),
            },
        })?;
        if line.trim() == "MODULES=()".to_owned().trim() {
            line = "MODULES=(btrfs)".to_owned();
        }
        lines.push(line);
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .map_err(|e| Error::OpenFile {
            source: e,
            context: "Failed to open /etc/mkinitcpio.conf".to_string(),
            backtrace: Trace {
                filename: file!(),
                function: "fn configure_bootloader() -> Result<()>",
                description: format!(
                    "OpenOptions::new().write(true).truncate(true).open({})",
                    path
                ),
            },
        })?;

    for line in lines {
        writeln!(file, "{}", line).map_err(|e| Error::WriteFile {
            source: e,
            context: "Failed to write to /etc/mkinitcpio.conf".to_string(),
            backtrace: Trace {
                filename: file!(),
                function: "fn configure_bootloader() -> Result<()>",
                description: format!("writeln!(file, \"{}\");", line),
            },
        })?;
    }

    Ok(())
}
