use std::process::Command;

use crate::functions::run_commands::run_command;
use crate::prelude::*;

pub fn set_hostname(hostname: &str) -> Result<()> {
    run_command(
        Command::new("sh")
            .arg("-c")
            .arg(format!("echo {} > /etc/hostname", hostname)),
    )?;

    let commands = [
        r#"echo "127.0.0.1    localhost" > /etc/hosts"#.to_string(),
        r#"echo "::1          localhost" >> /etc/hosts"#.to_string(),
        format!(
            r#"echo "127.0.1.1    {}.localdomain {}" >> /etc/hosts"#,
            hostname, hostname
        ),
    ];

    for command in commands {
        run_command(Command::new("sh").arg("-c").arg(command))?;
    }

    println!("Hosname successfully configured!");

    Ok(())
}
