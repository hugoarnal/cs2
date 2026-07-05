use std::fs;
use std::process::Command;

use anyhow::{anyhow, Result};

use crate::args::{get_jobs_number, Args};
use crate::shared;

enum BuildSystems {
    Makefile,
    Cmake,
}

impl BuildSystems {
    fn build(&self, args: &Args) -> Result<Vec<String>> {
        self.clean()?;

        let jobs = get_jobs_number(&args.jobs);

        let build_system_output = match *self {
            Self::Makefile => {
                // Running default `make`
                let mut command = Command::new("make");
                let env = shared::get_run_environment(args);

                command.arg(shared::envs_to_string(&env));
                command.envs(env);
                command.envs([("MAKEFLAGS", format!("-j{} -Otarget", jobs).as_str())]);

                let command = command.output()?;
                if !command.status.success() {
                    println!("Encountered an error while running make, continuing...");
                }

                shared::merge_outputs(command.stdout, command.stderr)
            }
            _ => return Err(anyhow!("Current build system is not supported")),
        };

        let command = Command::new("banana-check-repo").output()?;

        let all_output = if !command.status.success() {
            shared::merge_outputs(build_system_output, command.stdout)
        } else {
            build_system_output
        };

        shared::split_output(all_output)
    }

    fn clean(&self) -> Result<()> {
        match *self {
            Self::Makefile => {
                // TODO: add option to NOT clean
                println!("Running make fclean");

                let command = Command::new("make")
                    .arg("fclean")
                    .envs(shared::DEFAULT_RUN_ENV)
                    .status()?;

                if !command.success() {
                    println!("Error: Could not run rule 'fclean', trying 'clean'");
                    let command = Command::new("make")
                        .arg("clean")
                        .envs(shared::DEFAULT_RUN_ENV)
                        .status()?;
                    if !command.success() {
                        println!("Error: Could not run rule 'clean', continuing...");
                    }
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

pub fn find(args: &Args) -> Result<Vec<String>> {
    let paths = fs::read_dir("./")?;

    let mut build_system: Option<BuildSystems> = None;

    for path in paths {
        let file_name = path?.file_name().to_ascii_lowercase();
        if file_name == "makefile" || file_name == "gnumakefile" {
            build_system = Some(BuildSystems::Makefile);
        }
        if file_name == "cmakelists.txt" {
            build_system = Some(BuildSystems::Cmake);
        }
    }

    match build_system {
        Some(b) => b.build(args),
        None => Err(anyhow!(
            "Couldn't find build system, use \"cs2 run <command>\" instead",
        )),
    }
}
