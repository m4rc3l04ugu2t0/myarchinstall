use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
};

use crate::prelude::Result;

pub fn configure_bootloader() -> Result<()> {
    let path = "/etc/mkinitcpio.conf";
    let file = OpenOptions::new().read(true).open(path)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        let mut line = line?;
        if line.trim() == "MODULES=()".to_owned().trim() {
            line = "MODULES=(btrfs)".to_owned();
        }
        lines.push(line);
    }

    let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;

    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}
