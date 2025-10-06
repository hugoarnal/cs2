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
        match *self {
            Self::Makefile => {
                // TODO: add option to NOT clean
                let _ = Command::new("make")
                    .arg("fclean")
                    .envs(shared::DEFAULT_RUN_ENV)
                    .output()?;

                let command = Command::new("make")
                    .envs(shared::DEFAULT_RUN_ENV)
                    .output()?;

                if !command.status.success() {
                    println!("Encountered an error while running make, continuing...");
                }

                let all_output = shared::merge_outputs(command.stdout, command.stderr);

                Ok(shared::split_output(all_output)?)
            }
            _ => Err(Error::other("Couldn't find build system")),
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
