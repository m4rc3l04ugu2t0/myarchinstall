use crate::{
    error::Trace,
    prelude::{Error, Result},
};
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    process::Command,
};

use crate::functions::run_commands::run_command;

pub fn set_language(language: &[String]) -> Result<()> {
    edit_locale_gen(language)?;
    run_command(&mut Command::new("locale-gen"))?;

    configure_locale_conf(language)?;

    Ok(())
}

fn edit_locale_gen(language: &[String]) -> Result<()> {
    let locale_gen_path = "/etc/locale.gen";
    let file = OpenOptions::new()
        .read(true)
        .open(locale_gen_path)
        .map_err(|e| Error::OpenFile {
            source: e,
            context: format!("Failed to open file {}", locale_gen_path),
            backtrace: Trace {
                filename: file!(),
                function: "fn edit_locale_gen(language: &[String]) -> Result<()>",
                description: "OpenOptions::new().read(true).open(locale_gen_path)".to_string(),
            },
        })?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        let mut line = line.map_err(|e| Error::ReadFile {
            source: e,
            context: format!("Failed to read line from {}", locale_gen_path),
            backtrace: Trace {
                filename: file!(),
                function: "fn edit_locale_gen(language: &[String]) -> Result<()>",
                description: "reader.lines()".to_string(),
            },
        })?;
        if line.trim() == format!("#{}", language[0].trim()) {
            line = language[0].to_string();
        }
        lines.push(line);
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(locale_gen_path)
        .map_err(|e| Error::OpenFile {
            source: e,
            context: format!("Failed to open file {}", locale_gen_path),
            backtrace: Trace {
                filename: file!(),
                function: "fn edit_locale_gen(language: &[String]) -> Result<()>",
                description: "OpenOptions::new().write(true).truncate(true).open(locale_gen_path)"
                    .to_string(),
            },
        })?;

    for line in lines {
        writeln!(file, "{}", line).map_err(|e| Error::WriteFile {
            source: e,
            context: format!("Failed to write to file {}", locale_gen_path),
            backtrace: Trace {
                filename: file!(),
                function: "fn edit_locale_gen(language: &[String]) -> Result<()>",
                description: "writeln!(file, \"{}\", line)".to_string(),
            },
        })?;
    }

    Ok(())
}

fn configure_locale_conf(language: &[String]) -> Result<()> {
    let locale_conf_path = "/etc/locale.conf";
    let content = format!(
        "LANG={}\n",
        language[0].strip_suffix(" UTF-8").unwrap_or(&language[0])
    );

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(locale_conf_path)
        .map_err(|e| Error::OpenFile {
            source: e,
            context: format!("Failed to open file {}", locale_conf_path),
            backtrace: Trace {
                filename: file!(),
                function: "fn configure_locale_conf(language: &[String]) -> Result<()>",
                description: "OpenOptions::new().write(true).truncate(true).create(true).open(locale_conf_path)"
                    .to_string(),
            },
        })?;
    file.write_all(content.as_bytes())
        .map_err(|e| Error::WriteFile {
            source: e,
            context: format!("Failed to write to file {}", locale_conf_path),
            backtrace: Trace {
                filename: file!(),
                function: "fn configure_locale_conf(language: &[String]) -> Result<()>",
                description: "file.write_all(content.as_bytes())".to_string(),
            },
        })?;
    Ok(())
}
