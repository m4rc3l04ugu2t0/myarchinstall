use crate::prelude::*;
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    process::Command,
};

use crate::functions::run_commands::run_command;

pub fn set_language(language: &[String]) -> Result<()> {
    println!("Configurando linguagem do sistema...");

    edit_locale_gen(language)?;
    run_command(&mut Command::new("locale-gen"))?;

    configure_locale_conf(language)?;

    println!("Language successfully configured");
    Ok(())
}

fn edit_locale_gen(language: &[String]) -> Result<()> {
    let locale_gen_path = "/etc/locale.gen";
    let file = OpenOptions::new()
        .read(true)
        .open(locale_gen_path)
        .map_err(|e| Error::LocaleGen(format!("Failure to open {}: {}", locale_gen_path, e)))?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    let mut language_valid = false;

    for line in reader.lines() {
        let mut line = line.map_err(|e| Error::LocaleGen(format!("Failed to read line: {}", e)))?;
        if line.trim() == format!("#{}", language[0].trim()) {
            language_valid = true;
            line = language[0].to_string();
        }
        lines.push(line);
    }

    if !language_valid {
        return Err(Error::Static(
            "Language not found in /etc/locale.gen! Make sure it's correct.",
        ));
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(locale_gen_path)
        .map_err(|e| {
            Error::LocaleGen(format!(
                "Failure to open {} for reading: {}",
                locale_gen_path, e
            ))
        })?;

    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

fn configure_locale_conf(language: &[String]) -> Result<()> {
    let locale_conf_path = "/etc/locale.conf";
    let content = format!("LANG={}\n", language[0]);

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(locale_conf_path)
        .map_err(|e| {
            Error::LocaleGen(format!(
                "Failure to open {} from reading: {}",
                locale_conf_path, e
            ))
        })?;
    file.write_all(content.as_bytes()).map_err(|e| {
        Error::LocaleGen(format!("Failure to write in {}: {}", locale_conf_path, e))
    })?;
    Ok(())
}
