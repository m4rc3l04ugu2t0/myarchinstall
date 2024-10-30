use std::process::Command;

use crate::{functions::run_commands::run_command, ConfigureError};

pub fn set_keymaps(keymap: &str) -> Result<(), ConfigureError> {
    run_command(
        Command::new("sh")
            .arg("-c")
            .arg(format!("echo KEYMAP={} >> /etc/vconsole.conf", keymap)),
    )?;

    println!("Keymaps successfully configured!");

    Ok(())
}
