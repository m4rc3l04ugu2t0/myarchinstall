use tempfile::tempdir;

use crate::{functions::run_commands::run_command, prelude::Result};
use std::{
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Write},
    process::Command,
};

pub fn set_language(language: &[String]) -> Result<()> {
    edit_locale_gen(language)?;
    run_command(&mut Command::new("locale-gen"))?;

    configure_locale_conf(language)?;

    Ok(())
}

fn edit_locale_gen(language: &[String]) -> Result<()> {
    let tmp = tempdir().unwrap();
    let locale_gen_path = tmp.path().join("locale.gen");
    fs::write(&locale_gen_path, "#en_US.UTF-8 UTF-8").unwrap();

    let file = OpenOptions::new().read(true).open(&locale_gen_path)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    let mut languade_found = true;

    for line in reader.lines() {
        let mut line = line?;
        if line.trim() == format!("#{}", language[0].trim()) {
            languade_found = false;
            line = language[0].to_string();
        }
        lines.push(line);
    }

    if languade_found {
        return Err(crate::error::Error::Generic(format!(
            "Invalid language: {:?}",
            language
        )));
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&locale_gen_path)?;

    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

fn configure_locale_conf(language: &[String]) -> Result<()> {
    let tmp = tempdir().unwrap();
    let locale_conf_path = tmp.path().join("locale.gen");

    let content = format!(
        "LANG={}\n",
        language[0].strip_suffix(" UTF-8").unwrap_or(&language[0])
    );

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&locale_conf_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod test_set_language {
    use crate::configure_location::set_language::{configure_locale_conf, edit_locale_gen};

    #[test]
    fn test_locale_conf() {
        assert!(configure_locale_conf(&["teste".to_string()]).is_ok());
    }

    #[test]
    fn test_edit_locale_gen() {
        assert!(edit_locale_gen(&["en_US.UTF-8 UTF-8".to_string()]).is_ok());
        assert!(edit_locale_gen(&["en_US.UTF-8 UTF-8 Error".to_string()]).is_err());
    }
}
