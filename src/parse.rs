use std::fmt;
use std::io::Error;
use std::process::Command;
use std::str::FromStr;

use crate::shared;
use regex::Regex;

#[derive(Clone, PartialEq)]
enum ErrorLevel {
    FATAL,
    MAJOR,
    MINOR,
    INFO,
}

impl fmt::Display for ErrorLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
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

impl ErrorLevel {
    fn as_str(&self) -> &'static str {
        match *self {
            ErrorLevel::FATAL => "FATAL",
            ErrorLevel::MAJOR => "MAJOR",
            ErrorLevel::MINOR => "MINOR",
            ErrorLevel::INFO => "INFO",
        }
    }

    fn to_color(&self) -> shared::Colors {
        match *self {
            Self::FATAL => shared::Colors::RED,
            Self::MAJOR => shared::Colors::RED,
            Self::MINOR => shared::Colors::ORANGE,
            Self::INFO => shared::Colors::BLUE,
        }
    }

    fn to_color_str(&self) -> &'static str {
        self.to_color().as_str()
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
    ignore: bool,
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

fn skip_leading_dot(file: &str) -> &str {
    let mut chars = file.chars();
    // for some reason, chars.skip(2) did not work?
    chars.next();
    chars.next();
    chars.as_str()
}

fn parse_line(line: String) -> Option<LineError> {
    if !line.contains(shared::BANANA_ERROR_PREFIX) {
        return None;
    }

    let re = Regex::new(
        r"(?m)^([^:]+):?([0-9]*):?([0-9]*):.*(Minor|Major|Info|Fatal)] (.*?) \(([A-Z]-[A-Z][0-9]).*$",
    );
    for (_, [file, line_nb_s, col_nb_s, level_text, description, rule]) in re
        .expect("REASON")
        .captures_iter(&line)
        .map(|c| c.extract())
    {
        let line_nb: u32 = if line_nb_s.is_empty() {
            u32::MIN
        } else {
            line_nb_s.to_string().parse().unwrap()
        };
        let col_nb: u32 = if col_nb_s.is_empty() {
            u32::MIN
        } else {
            col_nb_s.to_string().parse().unwrap()
        };
        let file = if file.starts_with("./") {
            skip_leading_dot(file)
        } else {
            file
        };
        return Some(LineError {
            file: file.to_string(),
            line_nb: line_nb,
            col_nb: col_nb,
            level: ErrorLevel::from_str(level_text).unwrap(),
            rule: rule.to_string(),
            description: description.to_string(),
            ignore: false,
        });
    }
    None
}

fn summary_errors(errors: &Vec<LineError>) {
    let mut ignored_errors = 0;
    let mut errors_level = [
        (ErrorLevel::FATAL, 0),
        (ErrorLevel::MAJOR, 0),
        (ErrorLevel::MINOR, 0),
        (ErrorLevel::INFO, 0),
    ];

    for error in errors {
        if error.ignore {
            ignored_errors += 1;
            continue;
        }
        match error.level {
            ErrorLevel::FATAL => errors_level[0].1 += 1,
            ErrorLevel::MAJOR => errors_level[1].1 += 1,
            ErrorLevel::MINOR => errors_level[2].1 += 1,
            ErrorLevel::INFO => errors_level[3].1 += 1,
        };
    }

    if ignored_errors > 0 {
        println!(
            "{}{} ignored errors{} (use --no-ignore to see them)",
            shared::Colors::BOLD,
            ignored_errors,
            shared::Colors::RESET
        );
    }

    print!(
        "{}{} error(s){}: ",
        shared::Colors::BOLD,
        errors.len() - ignored_errors,
        shared::Colors::RESET
    );

    for (i, (level, amount)) in errors_level.iter().enumerate() {
        let bold = if *level == ErrorLevel::FATAL {
            shared::Colors::BOLD.as_str()
        } else {
            ""
        };

        let comma = if i < errors_level.len() - 1 { ", " } else { "" };

        // TODO: perhaps don't show if amount < 0
        print!(
            "{}{}{} {}{}{}",
            bold,
            level.to_color_str(),
            amount,
            level.as_str().to_ascii_lowercase(),
            shared::Colors::RESET,
            comma
        );
    }

    print!("\n");
}

fn print_errors(errors: &Vec<LineError>) {
    let mut prev_file_name = String::new();

    for error in errors {
        if error.ignore == true {
            continue;
        }

        if prev_file_name.is_empty() || prev_file_name != error.file {
            println!("{}:", error.file);
        }

        print!(
            "{}{} [{}]:{}",
            error.level.to_color_str(),
            error.level,
            error.rule,
            shared::Colors::RESET
        );
        print!(" {}", error.description);
        println!(
            "{}({}:{}:{}){}",
            shared::Colors::GRAY,
            error.file,
            error.line_nb,
            error.col_nb,
            shared::Colors::RESET
        );
        prev_file_name = error.file.clone();
    }

    summary_errors(errors);
}

fn verify_ignore(errors: &mut Vec<LineError>) -> Result<(), Error> {
    let command = Command::new("git").args(["clean", "-ndX"]).output()?;

    if !command.status.success() {
        // We're probably not in a git repo, no need to error out.
        return Ok(());
    }

    let ignored_files = String::from_utf8(command.stdout)
        .unwrap()
        .replace("Would remove ", "");

    for ignored_file in ignored_files
        .split("\n")
        .map(|f| String::from(f))
        .collect::<Vec<_>>()
    {
        for error in &mut *errors {
            if error.file == ignored_file {
                error.ignore = true;
            }
        }
    }

    Ok(())
}

/// remove duplicates by checking with PartialEq (dedup)
fn clean_errors_vector(errors: &mut Vec<LineError>) {
    errors.sort_by(|a, b| a.line_nb.cmp(&b.line_nb));
    errors.sort_by(|a, b| a.col_nb.cmp(&b.col_nb));
    errors.sort_by(|a, b| a.file.to_lowercase().cmp(&b.file.to_lowercase()));

    errors.dedup();
}

pub fn parse_output(lines: Vec<String>, dont_ignore: bool) -> Result<(), Error> {
    let mut errors: Vec<LineError> = Vec::new();

    for line in lines {
        let line_error = match parse_line(line) {
            Some(error) => error,
            None => continue,
        };

        errors.push(line_error);
    }

    if !dont_ignore {
        verify_ignore(&mut errors)?;
    }
    clean_errors_vector(&mut errors);
    print_errors(&errors);

    Ok(())
}
