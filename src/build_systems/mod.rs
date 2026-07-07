mod makefile;

use std::fs;
use std::process::Command;

use anyhow::{anyhow, Result};

use crate::args::{get_jobs_number, Args};
use crate::shared;

pub trait BuildSystems {
    fn build(&self, args: &Args) -> Result<Vec<u8>>;
    fn clean(&self) -> Result<()>;
}

pub fn build(build_system: &dyn BuildSystems, args: &Args) -> Result<Vec<String>> {
    build_system.clean()?;

    let build_system_output = build_system.build(args)?;

    let command = Command::new("banana-check-repo").output()?;

    let all_output = if !command.status.success() {
        shared::merge_outputs(build_system_output, command.stdout)
    } else {
        build_system_output
    };

    shared::split_output(all_output)
}

pub fn construct(args: &Args) -> Result<Box<dyn BuildSystems>> {
    let paths = fs::read_dir("./")?;

    for path in paths {
        let file_name = path?.file_name().to_ascii_lowercase();
        if file_name == "makefile" || file_name == "gnumakefile" {
            return Ok(Box::new(makefile::Makefile::new(get_jobs_number(
                &args.jobs,
            ))));
        }
    }
    Err(anyhow!("Couldn't find "))
}
