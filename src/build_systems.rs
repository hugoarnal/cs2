use std::fs;
use std::io::Error;
use std::process::Command;

use crate::shared;

// TODO: use enums

fn build_makefile() -> Result<Vec<String>, Error> {
    let command = Command::new("make")
        .envs(shared::DEFAULT_RUN_ENV)
        .output()?;

    if !command.status.success() {
        return Err(Error::other("Error occured"));
    }

    let all_output = shared::merge_outputs(command.stdout, command.stderr);

    Ok(shared::split_output(all_output)?)
}

pub fn find() -> Result<Vec<String>, Error> {
    let paths = fs::read_dir("./")?;

    let mut str: Option<Vec<String>> = None;

    for path in paths {
        let file_name = path?.file_name().to_ascii_lowercase();
        if file_name == "makefile" || file_name == "gnumakefile" {
            str = Some(build_makefile()?);
        }
        if file_name == "cmakelists.txt" {
            break;
        }
    }

    match str {
        Some(r) => Ok(r),
        None => Err(Error::other("Couldn't find build system")),
    }
}
