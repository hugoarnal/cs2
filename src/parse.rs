use std::io::Error;

use crate::shared;

pub fn parse_output(lines: Vec<String>) -> Result<(), Error> {
    for line in lines {
        if line.contains(shared::BANANA_ERROR_PREFIX) {
            println!("line: {}", line);
        }
    }
    Ok(())
}
