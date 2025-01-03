use std::process::Command;

use tempfile::tempdir;

use crate::{functions::run_commands::run_command, prelude::Result};

pub fn set_keymaps(keymap: &str) -> Result<()> {
    let tmp = tempdir().unwrap();
    let vconsole_path = tmp.path().join("vconsole.conf");
    run_command(
        Command::new("sh")
            .arg("-c")
            .arg(format!("echo KEYMAP={} >> {:?}", keymap, vconsole_path)),
    )?;
    Ok(())
}

#[cfg(test)]
mod test_keymaps {
    use crate::{
        configure_keymaps::set_keymaps::set_keymaps, structure_config::config_path::config_paths,
    };

    #[test]
    fn test_set_keymaps() {
        config_paths().unwrap();
        assert!(set_keymaps("br-abnt2").is_ok());
    }
}
