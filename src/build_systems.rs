use std::fs;
use std::io::Error;
use std::path::Path;
use std::process::Command;

use crate::package::Packages;
use crate::shared;

enum BuildSystems {
    Makefile,
    Cmake,
    None,
}

impl BuildSystems {
    fn build(&self, parallelism: String) -> Result<Vec<String>, Error> {
        self.clean()?;

        match *self {
            Self::Makefile => {
                // Running default `make`
                let mut command = Command::new("make");
                command.envs(shared::DEFAULT_RUN_ENV);
                command.envs([("MAKEFLAGS", format!("-j{} -Otarget", parallelism).as_str())]);

                let command = command.output()?;
                if !command.status.success() {
                    println!("Encountered an error while running make, continuing...");
                }

                let both_std_output = shared::merge_outputs(command.stdout, command.stderr);

                let command = Command::new("banana-check-repo").output()?;

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

    build_system.build(parallelism)
}
