use std::{io::Error, path::Path, process::Command};

use clap::ArgMatches;

use crate::commands::shared;

fn pull_repo(path: &str, package: &str) -> Result<bool, Error> {
    let command = format!("cd {} && git pull origin main", path);

    let results = Command::new("sh").args(["-c", &command]).output()?;

    if !results.status.success() {
        return Err(Error::other(format!("Had problems updating {}", package)));
    };

    // absolute cinema
    if String::from_utf8(results.stdout)
        .unwrap()
        .contains("Already up to date.")
    {
        Ok(false)
    } else {
        Ok(true)
    }
}

fn update_package(package: &str, parallelism: bool, force: bool) -> Result<(), Error> {
    let path = shared::get_final_path(package);

    if !Path::new(&path).exists() {
        return Err(Error::other(format!(
            "Impossible to find {}, have you installed it with cs2 install?",
            package
        )));
    }

    if pull_repo(&path, package)? || force {
        shared::build_package(package, parallelism)?;
    } else {
        println!("Nothing to update");
    }

    Ok(())
}

fn get_args_amount(args: &ArgMatches, all_args: &[&'static str]) -> u16 {
    let mut i = 0;

    for arg in all_args {
        if args.get_flag(arg) {
            i += 1;
        }
    }
    i
}

pub fn handler(args: &ArgMatches) -> Result<(), Error> {
    let all_args = ["cs2", "epiclang", "banana", "parallelism", "force"];
    let valid_args = ["cs2", "epiclang", "banana"];

    let parallelism = args.get_flag("parallelism");
    let force = args.get_flag("force");
    let has_optional_arg = parallelism || force;

    if !args.args_present() || (get_args_amount(args, &all_args) <= 2 && has_optional_arg) {
        for arg in valid_args {
            println!("Updating {}", arg);
            update_package(arg, parallelism, force)?;
        }
        return Ok(());
    }

    for valid_arg in valid_args {
        if args.get_flag(valid_arg) {
            println!("Updating only {}", valid_arg);
            update_package(valid_arg, parallelism, force)?;
        };
    }

    Ok(())
}
