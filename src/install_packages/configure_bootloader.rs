use std::{
    backtrace::Backtrace,
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
};

use crate::prelude::{Error, Result};

pub fn configure_bootloader() -> Result<()> {
    let path = "/etc/mkinitcpio.conf";
    let file = OpenOptions::new()
        .read(true)
        .open(path)
        .map_err(|e| Error::OpenFile {
            source: e.into(),
            context: "Failed to open /etc/mkinitcpio.conf".to_string(),
            backtrace: Backtrace::capture(),
        })?;

    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        let mut line = line.map_err(|e| Error::ReadFile {
            source: e.into(),
            context: "Failed to read line from /etc/mkinitcpio.conf".to_string(),
            backtrace: Backtrace::capture(),
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
            source: e.into(),
            context: "Failed to open /etc/mkinitcpio.conf".to_string(),
            backtrace: Backtrace::capture(),
        })?;

    for line in lines {
        writeln!(file, "{}", line).map_err(|e| Error::WriteFile {
            source: e.into(),
            context: "Failed to write to /etc/mkinitcpio.conf".to_string(),
            backtrace: Backtrace::capture(),
        })?;
    }

    Ok(())
}
