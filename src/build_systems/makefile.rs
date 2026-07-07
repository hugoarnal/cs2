use anyhow::Result;
use std::process::Command;

use crate::args::Args;
use crate::{build_systems::BuildSystems, shared};

pub struct Makefile {
    jobs: String,
}

impl Makefile {
    pub fn new(jobs: String) -> Makefile {
        Makefile { jobs }
    }
}

impl BuildSystems for Makefile {
    fn build(&self, args: &Args) -> Result<Vec<u8>> {
        // Running default `make`
        let mut command = Command::new("make");
        let env = shared::get_run_environment(args);

        command.arg(shared::envs_to_string(&env));
        command.envs(env);
        command.envs([("MAKEFLAGS", format!("-j{} -Otarget", self.jobs).as_str())]);

        let command = command.output()?;
        if !command.status.success() {
            println!("Encountered an error while running make, continuing...");
        }

        Ok(shared::merge_outputs(command.stdout, command.stderr))
    }

    fn clean(&self) -> Result<()> {
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
}
