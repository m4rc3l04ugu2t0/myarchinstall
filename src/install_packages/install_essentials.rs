use std::process::Command;

use crate::{
    functions::run_commands::run_command,
    install_packages::configure_bootloader::configure_bootloader,
};

use crate::prelude::*;

pub fn install_assentials(packages: &[String]) -> Result<()> {
    run_command(
        Command::new("pacman")
            .arg("-S")
            .args(packages)
            .arg("--noconfirm"),
    )?;
    configure_bootloader()?;

    run_command(
        Command::new("pacman")
            .arg("-S")
            .arg("grub")
            .arg("efibootmgr")
            .arg("--noconfirm"),
    )?;

    run_command(Command::new("grub-install").args([
        "--target=x86_64-efi",
        "--efi-directory=/boot",
        "--bootloader-id=rustinstallarch",
        "--recheck",
    ]))?;

    run_command(Command::new("grub-mkconfig").args(["-o", "/boot/grub/grub.cfg"]))?;

    run_command(Command::new("cat").arg("/boot/grub/grub.cfg"))?;

    Ok(())
}
