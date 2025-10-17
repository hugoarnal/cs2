use std::fs;
use std::io::Error;
use std::path::Path;
use std::process::Command;

use crate::package::Packages;
use crate::shared;

enum BuildSystems {
    Makefile,
    Cmake,
}

impl BuildSystems {
    fn build(&self, parallelism: String) -> Result<Vec<String>, Error> {
        self.clean()?;

        let build_system_output = match *self {
            Self::Makefile => {
                // Running default `make`
                let mut command = Command::new("make");
                command.envs(shared::DEFAULT_RUN_ENV);
                command.envs([("MAKEFLAGS", format!("-j{} -Otarget", parallelism).as_str())]);

                let command = command.output()?;
                if !command.status.success() {
                    println!("Encountered an error while running make, continuing...");
                }

                shared::merge_outputs(command.stdout, command.stderr)
            }
            _ => return Err(Error::other("Current build system is not supported")),
        };

        let command = Command::new("banana-check-repo").output()?;

        let all_output = if !command.status.success() {
            shared::merge_outputs(build_system_output, command.stdout)
        } else {
            build_system_output
        };

        shared::split_output(all_output)
    }

    fn clean(&self) -> Result<(), Error> {
        match *self {
            Self::Makefile => {
                // TODO: add option to NOT clean
                println!("Running make fclean");

                let command = Command::new("make")
                    .arg("fclean")
                    .envs(shared::DEFAULT_RUN_ENV)
                    .spawn()?
                    .wait_with_output();

                if !command.unwrap().status.success() {
                    println!("Error: Could not run rule 'fclean', trying 'clean'");
                    let command_fallback = Command::new("make").arg("clean").envs(shared::DEFAULT_RUN_ENV).spawn()?.wait_with_output();
                    if !command_fallback.unwrap().status.success() {
                        println!("Error: Could not run rule 'clean', continuing...");
                    }
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

pub fn verify_packages() -> bool {
    let packages = [Packages::Epiclang, Packages::Banana];

    for package in packages {
        let mut found = false;

        for path in package.get_packages() {
            if Path::new(path).exists() {
                found = true;
            }
        }

        if !found {
            println!("Couldn't find {}", package);
            return false;
        }
    }
    true
}

pub fn find(parallelism: String) -> Result<Vec<String>, Error> {
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
        Some(b) => b.build(parallelism),
        None => Err(Error::other(
            "Couldn't find build system, use \"cs2 run <command>\" instead",
        )),
    }
}
