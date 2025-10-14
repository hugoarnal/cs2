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
    line_nb: Option<u32>,
    col_nb: Option<u32>,
    level: ErrorLevel,
    rule: String,
    description: String,
    ignore: bool,
    occurences: u32,
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
    for (_, [file, line_nb, col_nb, level_text, description, rule]) in re
        .expect("REASON")
        .captures_iter(&line)
        .map(|c| c.extract())
    {
        let line_nb: Option<u32> = if line_nb.is_empty() {
            None
        } else {
            Some(line_nb.to_string().parse().unwrap())
        };
        let col_nb: Option<u32> = if col_nb.is_empty() {
            None
        } else {
            Some(col_nb.to_string().parse().unwrap())
        };
        let file = if file.starts_with("./") {
            skip_leading_dot(file)
        } else {
            file
        };
        return Some(LineError {
            file: file.to_string(),
            line_nb,
            col_nb,
            level: ErrorLevel::from_str(level_text).unwrap(),
            rule: rule.to_string(),
            description: description.to_string(),
            ignore: false,
            occurences: 1,
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
    let mut nb_errors: u32 = 0;

    for error in errors {
        if error.ignore {
            ignored_errors += 1;
            continue;
        }
        nb_errors += 1;
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

    // TODO: Add trollface when I get the approbation
    if nb_errors == 0 {
        println!(
            "✅ {}There are no coding style errors!{}",
            shared::Colors::BOLD,
            shared::Colors::RESET
        );
        return;
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
            println!(
                "{}{}:{}",
                shared::Colors::BOLD,
                error.file,
                shared::Colors::RESET
            );
        }

        print!(
            "{}{} [{}]:{}",
            error.level.to_color_str(),
            error.level,
            error.rule,
            shared::Colors::RESET
        );
        print!(" {} ", error.description);
        print!("{}", shared::Colors::GRAY);
        print!("({}", error.file);
        match error.line_nb {
            Some(line_nb) => {
                print!(":{}", line_nb);
            }
            None => {}
        }
        match error.col_nb {
            Some(col_nb) => {
                print!(":{}", col_nb);
            }
            None => {}
        }
        print!(")");
        if error.occurences > 1 {
            print!(" (x{})", error.occurences)
        }
        println!("{}", shared::Colors::RESET);
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

// Making alternative for Vec.dedup() in order to count the number of occurences
// The function is kinda disgusting and probably not very optimized (.remove() is on O(N))
fn my_dedup(errors: &mut Vec<LineError>) {
    let mut len: usize = errors.len();
    if len <= 0 {
        return;
    }
    let mut temp: LineError = errors[0].clone();
    let mut i: usize = 1;
    while i < len - 1 {
        if temp == errors[i] {
            errors[i - 1].occurences += 1;
            errors.remove(i);
            len -= 1;
        } else {
            temp = errors[i].clone();
            i += 1;
        }
    }
}

/// remove duplicates by checking with PartialEq (dedup)
fn clean_errors_vector(errors: &mut Vec<LineError>) {
    errors.sort_by(|a, b| a.line_nb.cmp(&b.line_nb));
    errors.sort_by(|a, b| a.col_nb.cmp(&b.col_nb));
    errors.sort_by(|a, b| a.file.to_lowercase().cmp(&b.file.to_lowercase()));

    my_dedup(errors);
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
