use anyhow::{anyhow, Result};
use std::process::Command;

use crate::{parse, shared};

pub fn run(command_args: &[String]) -> Result<()> {
    let mut i = command_args.iter();

    let program = i.next().unwrap();

    let outputs = Command::new(program)
        .envs(shared::DEFAULT_RUN_ENV)
        .args(i.collect::<Vec<_>>())
        .output()?;

    let all_output = shared::merge_outputs(outputs.stdout, outputs.stderr);

    parse::parse_output(shared::split_output(all_output)?, true, None)?;

    if !outputs.status.success() {
        return Err(anyhow!(
            "Received error code: {}",
            outputs.status.code().unwrap()
        ));
    }

    Ok(())
}
