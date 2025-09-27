use std::io::Error;

use clap::ArgMatches;

pub fn handler(args: &ArgMatches) -> Result<(), Error> {
    let _ = args;
    return Ok(());
}
