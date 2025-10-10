use std::{io::Error, process::Command};

use crate::{parse, shared};

pub fn run(command_args: Vec<&String>) -> Result<(), Error> {
    let mut i = command_args.iter();

    let program = i.next().unwrap();

    let outputs = Command::new(program)
        .envs(shared::DEFAULT_RUN_ENV)
        .args(i.collect::<Vec<_>>())
        .output()?;

    let all_output = shared::merge_outputs(outputs.stdout, outputs.stderr);

    parse::parse_output(shared::split_output(all_output)?, true)?;

    if !outputs.status.success() {
        return Err(Error::other(format!(
            "Received error code: {}",
            outputs.status.code().unwrap()
        )));
    }

    Ok(())
}
