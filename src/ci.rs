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
                    if error.ignore {
                        continue;
                    }
                    print!("::error file={},", error.file);

                    if let Some(nb) = error.line_nb {
                        print!("line={},", nb)
                    }
                    if let Some(nb) = error.col_nb {
                        print!("col={},", nb)
                    }

                    print!("title={} [{}]::", error.level, error.rule);
                    println!("{}", error.description);
                }
            }
        }
    }
}
