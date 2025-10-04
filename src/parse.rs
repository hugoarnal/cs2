use std::fmt;
use std::io::Error;
use std::str::FromStr;

use crate::shared;

enum ErrorLevel {
    FATAL,
    MAJOR,
    MINOR,
    INFO,
}

impl fmt::Display for ErrorLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level = match *self {
            ErrorLevel::FATAL => "FATAL",
            ErrorLevel::MAJOR => "MAJOR",
            ErrorLevel::MINOR => "MINOR",
            ErrorLevel::INFO => "INFO",
        };
        write!(f, "{}", level)
    }
}

impl FromStr for ErrorLevel {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Error> {
        match input {
            "Fatal" => Ok(Self::FATAL),
            "Major" => Ok(Self::MAJOR),
            "Minor" => Ok(Self::MINOR),
            "Info" => Ok(Self::INFO),
            _ => Err(Error::other("Couldn't find error type")),
        }
    }
}

pub struct LineError {
    file: String,
    line_nb: u32,
    col_nb: u32,
    level: ErrorLevel,
    rule: String,
    description: String,
}

fn parse_line(line: String) -> Option<LineError> {
    if !line.contains(shared::BANANA_ERROR_PREFIX) {
        return None;
    }

    // TODO: replace this line parsing with regex
    // Behold, the worst code I've ever writted in my life.

    let mut split_semi = line.split(":");

    let file = split_semi.nth(0).unwrap().to_string();
    let line_nb: u32 = split_semi.nth(0).unwrap().to_string().parse().unwrap();
    let col_nb: u32 = split_semi.nth(0).unwrap().to_string().parse().unwrap();

    let mut split_right_bracket = line.split("]");

    // skull emoji
    let first_split = split_right_bracket.nth(1).unwrap().to_string();
    let level_text = first_split.split("[").nth(1).unwrap();
    let level = ErrorLevel::from_str(level_text).unwrap();

    let split_parenthesis = line.split("(");
    let rule = split_parenthesis
        .last()
        .unwrap()
        .split(")")
        .nth(0)
        .unwrap()
        .to_string();

    let description = line
        .split("] ")
        .last()
        .unwrap()
        .split("(")
        .nth(0)
        .unwrap()
        .to_string();

    Some(LineError {
        file: file,
        line_nb: line_nb,
        col_nb: col_nb,
        level: level,
        rule: rule,
        description: description,
    })
}

pub fn parse_output(lines: Vec<String>) -> Result<(), Error> {
    for line in lines {
        let line_error = match parse_line(line) {
            Some(error) => error,
            None => continue,
        };

        println!("file: {}", line_error.file);
        println!("line_nb: {}", line_error.line_nb);
        println!("col_nb: {}", line_error.col_nb);
        println!("level: {}", line_error.level);
        println!("rule: {}", line_error.rule);
        println!("desc: {}", line_error.description);
        println!("-----------------");
    }
    Ok(())
}
