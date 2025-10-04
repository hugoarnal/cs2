use std::fmt;
use std::io::Error;
use std::str::FromStr;

use crate::shared;

#[derive(Clone, PartialEq)]
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

#[derive(Clone)]
pub struct LineError {
    file: String,
    line_nb: u32,
    col_nb: u32,
    level: ErrorLevel,
    rule: String,
    description: String,
}

/// Check for equality in file, line & col nb, level and rule
/// We don't check for the description as it might be different
impl PartialEq for LineError {
    fn eq(&self, rhs: &LineError) -> bool {
        return self.file == rhs.file
            && self.line_nb == rhs.line_nb
            && self.col_nb == rhs.col_nb
            && self.level == rhs.level
            && self.rule == rhs.rule;
    }
}

fn parse_line(line: String) -> Option<LineError> {
    if !line.contains(shared::BANANA_ERROR_PREFIX) {
        return None;
    }

    // TODO: replace this line parsing with regex
    // Behold, the worst code I've ever writted in my life.

    let mut split_semi = line.split(":");

    let file = split_semi.next().unwrap().to_string();
    let line_nb: u32 = split_semi.next().unwrap().to_string().parse().unwrap();
    let col_nb: u32 = split_semi.next().unwrap().to_string().parse().unwrap();

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
        .next()
        .unwrap()
        .to_string();

    let description = line
        .split("] ")
        .last()
        .unwrap()
        .split("(")
        .next()
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

fn print_error(errors: &Vec<LineError>) {
    for error in errors {
        println!("file: {}", error.file);
        println!("line_nb: {}", error.line_nb);
        println!("col_nb: {}", error.col_nb);
        println!("level: {}", error.level);
        println!("rule: {}", error.rule);
        println!("desc: {}", error.description);
        println!("-----------------");
    }
}

pub fn parse_output(lines: Vec<String>) -> Result<(), Error> {
    let mut errors: Vec<LineError> = Vec::new();

    for line in lines {
        let line_error = match parse_line(line) {
            Some(error) => error,
            None => continue,
        };

        errors.push(line_error);
    }

    // print_error(&errors);
    // verify_errors(errors)?;
    errors.dedup();
    print_error(&errors);

    Ok(())
}
