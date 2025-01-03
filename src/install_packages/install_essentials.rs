use std::fs::create_dir_all;
use std::process::Command;

use log::info;
use tempfile::tempdir;

use crate::functions::run_commands::run_command;

use crate::prelude::Result;

pub fn install_assentials(packages: &[String]) -> Result<()> {
    let tmp_dir = tempdir().unwrap();
    let efi_directory = tmp_dir.path().join("efi");
    create_dir_all(&efi_directory)?;
    run_command(
        Command::new("pacman")
            .arg("-S")
            .args(packages)
            .arg("--noconfirm"),
    )?;

    info!("Skip config bootloader to tests");
    // configure_bootloader()?;

    // run_command(
    //     Command::new("pacman")
    //         .arg("-S")
    //         .arg("grub")
    //         .arg("efibootmgr")
    //         .arg("--noconfirm"),
    // )?;

    // run_command(Command::new("grub-install").args([
    //     "--target=x86_64-efi",
    //     &format!("--efi-directory={}", efi_directory.display()),
    //     "--bootloader-id=rustinstallarch",
    //     "--recheck",
    // ]))?;

    // run_command(Command::new("grub-mkconfig").args(["-o", "/boot/grub/grub.cfg"]))?;

    // run_command(Command::new("cat").arg("/boot/grub/grub.cfg"))?;

    Ok(())
}
