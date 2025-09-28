use std::io::Error;
use std::process::Command;

pub fn find() -> Result<Vec<String>, Error> {
    let version_output = Command::new("make").output()?;

    Err(Error::other("yeah"))
}
