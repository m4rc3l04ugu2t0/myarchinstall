use crate::{functions::run_password_command::run_passwd_command, ConfigureError};

pub fn set_root(password: &str) -> Result<(), ConfigureError> {
    println!("Enter the new password for the root user:");

    run_passwd_command(password, "root")?;

    println!("Successfully!");
    Ok(())
}
