use std::fmt;
use std::path::Path;
use std::process::Command;

use anyhow::{anyhow, Result};

use crate::args::Args;

pub const BANANA_ERROR_PREFIX: &str = "[Banana] ";
pub const DEFAULT_RUN_ENV: [(&str, &str); 1] = [("CC", "epiclang")];

#[allow(clippy::upper_case_acronyms)]
pub enum Colors {
    GRAY,
    RED,
    ORANGE,
    BLUE,
    BOLD,
    RESET,
}

impl Colors {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Self::GRAY => "\x1b[0;90m",
            Self::RED => "\x1b[0;31m",
            Self::ORANGE => "\x1b[0;93m",
            Self::BLUE => "\x1b[0;36m",
            Self::BOLD => "\x1b[0;01m",
            Self::RESET => "\x1b[0;0m",
        }
    }
}

impl fmt::Display for Colors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub fn split_output(output: Vec<u8>) -> Result<Vec<String>> {
    let output_str = String::from_utf8(output)?;

    Ok(output_str.split("\n").map(String::from).collect::<Vec<_>>())
}

pub fn merge_outputs(stdout: Vec<u8>, stderr: Vec<u8>) -> Vec<u8> {
    let mut merged: Vec<u8> = Vec::with_capacity(stdout.len() + stderr.len());

    stdout.iter().for_each(|c| merged.push(*c));
    stderr.iter().for_each(|c| merged.push(*c));

    merged
}

/// similar to fs::create_dir_all except with sudo privileges
pub fn create_directory(path: &str) -> Result<()> {
    if Path::new(&path).exists() {
        return Ok(());
    };

    match Command::new("sudo").args(["mkdir", "-p", path]).status() {
        Ok(_) => Ok(()),
        Err(_) => Err(anyhow!("Couldn't create folder")),
    }
}

pub fn get_run_environment(args: &Args) -> [(&str, String); 1] {
    let mut epiclang_command = String::from("epiclang");

    if let Some(rules) = &args.ignore_rules {
        println!("Ignoring the following rules: {}", rules);
        epiclang_command.push_str(format!(" -fplugin-arg-banana-ignore-rules={}", rules).as_str());
    }

    if let Some(paths) = &args.ignore_paths {
        println!("Ignoring the following paths: {}", paths);
        epiclang_command.push_str(format!(" -fplugin-arg-banana-ignore-paths={}", paths).as_str());
    }

    [("CC", epiclang_command)]
}
