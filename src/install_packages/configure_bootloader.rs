use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
};

use crate::ConfigureError;

pub fn configure_bootloader() -> Result<(), ConfigureError> {
    let path = "/etc/mkinitcpio.conf";
    let file = OpenOptions::new()
        .read(true)
        .open(path)
        .map_err(|e| ConfigureError::Bootloader(format!("Failure to open {}: {}", path, e)))?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        let mut line =
            line.map_err(|e| ConfigureError::Bootloader(format!("Failed to read line: {}", e)))?;
        if line.trim() == "MODULES=()".to_owned().trim() {
            line = "MODULES=(btrfs)".to_owned();
        }
        lines.push(line);
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .map_err(|e| {
            ConfigureError::Bootloader(format!("Failure to open {} for reading: {}", path, e))
        })?;

    for line in lines {
        writeln!(file, "{}", line).map_err(|e| {
            ConfigureError::Bootloader(format!("Failure to whire file {}, {}", path, e))
        })?;
    }

    Ok(())
}
