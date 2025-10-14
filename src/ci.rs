use std::io::Error;
use std::str::FromStr;

use crate::parse::LineError;

pub enum Ci {
    GitHub,
}

impl FromStr for Ci {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Error> {
        match input.to_ascii_lowercase().as_str() {
            "github" => Ok(Self::GitHub),
            _ => Err(Error::other("Unknown CI platform")),
        }
    }
}

impl Ci {
    pub fn print_errors(&self, errors: &Vec<LineError>) {
        match *self {
            Self::GitHub => {
                for error in errors {
                    print!("::error file={},", error.file);

                    match error.line_nb {
                        Some(nb) => print!("line={},", nb),
                        None => {}
                    }
                    match error.col_nb {
                        Some(nb) => print!("col={},", nb),
                        None => {}
                    }

                    print!("title={} [{}]::", error.level, error.rule);
                    println!("{}", error.description);
                }
            }
        }
    }
}
