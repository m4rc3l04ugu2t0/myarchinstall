use std::process::Command;

use tempfile::tempdir;

use crate::functions::run_commands::run_command;
use crate::prelude::Result;

pub fn set_hostname(hostname: &str) -> Result<()> {
    let tmp = tempdir().unwrap();
    let hosts_path = tmp.path().join("hosts");
    let hostname_path = tmp.path().join("hostsname");

    run_command(Command::new("sh").arg("-c").arg(format!(
        "echo {} > {}",
        hostname,
        hostname_path.to_str().unwrap()
    )))?;

    let commands = [
        format!(r#"echo "127.0.0.1    localhost" > {:?}"#, hosts_path),
        format!(r#"echo "::1          localhost" >> {:?}"#, hosts_path),
        format!(
            r#"echo "127.0.1.1    {}.localdomain {}" >> {:?}"#,
            hostname, hostname, hosts_path
        ),
    ];

    for command in commands {
        run_command(Command::new("sh").arg("-c").arg(command))?;
    }

    Ok(())
}

#[cfg(test)]
mod test_hostname {
    use crate::structure_config::config_path::config_paths;

    use super::*;

    #[test]
    fn test_set_hostname() {
        config_paths().unwrap();
        assert!(set_hostname("test").is_ok());
    }
}
