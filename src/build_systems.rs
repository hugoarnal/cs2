use std::fs;
use std::io::Error;
use std::process::Command;

use crate::shared;

enum BuildSystems {
    Makefile,
    Cmake,
    None,
}

impl BuildSystems {
    fn build(&self) -> Result<Vec<String>, Error> {
        self.clean()?;

        match *self {
            Self::Makefile => {
                // Running default `make`
                let command = Command::new("make")
                    .envs(shared::DEFAULT_RUN_ENV)
                    .output()?;

                if !command.status.success() {
                    println!("Encountered an error while running make, continuing...");
                }

                let both_std_output = shared::merge_outputs(command.stdout, command.stderr);

                // Run `banana-check-repo-cs2`
                let command = Command::new("banana-check-repo-cs2").output()?;

                let all_output = if !command.status.success() {
                    shared::merge_outputs(both_std_output, command.stdout)
                } else {
                    both_std_output
                };

                Ok(shared::split_output(all_output)?)
            }
            _ => Err(Error::other("Couldn't find build system")),
        }
    }

    fn clean(&self) -> Result<(), Error> {
        match *self {
            Self::Makefile => {
                // TODO: add option to NOT clean
                println!("Running make fclean");

                let _ = Command::new("make")
                    .arg("fclean")
                    .envs(shared::DEFAULT_RUN_ENV)
                    .spawn()?
                    .wait();

                Ok(())
            }
            _ => Ok(()),
        }
    }
}

pub fn find() -> Result<Vec<String>, Error> {
    let paths = fs::read_dir("./")?;

    let mut build_system = BuildSystems::None;

    for path in paths {
        let file_name = path?.file_name().to_ascii_lowercase();
        if file_name == "makefile" || file_name == "gnumakefile" {
            build_system = BuildSystems::Makefile;
        }
        if file_name == "cmakelists.txt" {
            build_system = BuildSystems::Cmake;
        }
    }

    build_system.build()
}
