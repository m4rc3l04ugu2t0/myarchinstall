use std::{
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Write},
};

use tempfile::tempdir;

use crate::prelude::Result;

pub fn configure_bootloader() -> Result<()> {
    let tmp = tempdir().unwrap();
    let mkinitcpio_path = tmp.path().join("mkinitcpio.conf");
    fs::write(&mkinitcpio_path, "MODULES=()").unwrap();
    let file = OpenOptions::new().read(true).open(&mkinitcpio_path)?;

    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    let mut okay = true;

    for line in reader.lines() {
        let mut line = line?;
        if line.trim() == "MODULES=()".to_owned().trim() {
            okay = false;
            line = "MODULES=(btrfs)".to_owned();
        }
        lines.push(line);
    }

    if okay {
        return Err(crate::error::Error::Generic(format!(
            "Error in {:?} file",
            mkinitcpio_path
        )));
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(mkinitcpio_path)?;

    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

#[test]
fn test_configure_bootloader() {
    assert!(configure_bootloader().is_ok());
}
