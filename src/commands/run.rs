use std::{io::Error, process::Command};

use crate::shared;

pub fn run(command_args: Vec<&String>) -> Result<(), Error> {
    let mut i = command_args.iter();

    let program = i.nth(0).unwrap();

    let outputs = Command::new(program)
        .envs(shared::DEFAULT_RUN_ENV)
        .args(i.collect::<Vec<_>>())
        .output()?;

    for line in shared::split_output(outputs.stderr)? {
        if line.contains(shared::BANANA_ERROR_PREFIX) {
            println!("line: {}", line);
        }
    }

    if !outputs.status.success() {
        return Err(Error::other(format!(
            "Received error code: {}",
            outputs.status.code().unwrap()
        )));
    }

    Ok(())
}
