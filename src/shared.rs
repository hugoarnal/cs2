use std::fmt;
use std::io::Error;
use std::path::Path;
use std::process::Command;

pub const BANANA_ERROR_PREFIX: &str = "[Banana] ";
pub const DEFAULT_RUN_ENV: [(&str, &str); 1] = [("CC", "epiclang")];

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

pub fn split_output(output: Vec<u8>) -> Result<Vec<String>, Error> {
    // TODO: replace unwrap if possible
    let output_str = String::from_utf8(output).unwrap();

    Ok(output_str
        .split("\n")
        .map(|f| String::from(f))
        .collect::<Vec<_>>())
}

pub fn merge_outputs(stdout: Vec<u8>, stderr: Vec<u8>) -> Vec<u8> {
    let mut merged: Vec<u8> = Vec::new();
    merged.reserve(stdout.len() + stderr.len());

    stdout.iter().for_each(|c| merged.push(*c));
    stderr.iter().for_each(|c| merged.push(*c));

    merged
}

/// similar to fs::create_dir_all except with sudo privileges
pub fn create_directory(path: &str) -> Result<(), Error> {
    if Path::new(&path).exists() {
        return Ok(());
    };

    match Command::new("sudo").args(["mkdir", "-p", &path]).status() {
        Ok(_) => {
            return Ok(());
        }
        Err(e) => return Err(e),
    };
}
