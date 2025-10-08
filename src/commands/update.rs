use std::{io::Error, path::Path, process::Command};

use clap::ArgMatches;

use crate::commands::shared;

/// Returns true if project needs to be rebuilt, false if it's already at the latest version
fn pull_repo(path: &str, package: &str) -> Result<bool, Error> {
    let command = format!("cd {} && git pull origin main", path);
    let results = Command::new("sh").args(["-c", &command]).output()?;

    if !results.status.success() {
        let command = format!(
            "cd {} && git reset --hard main && git pull origin main",
            path
        );
        let results = Command::new("sh").args(["-c", &command]).output()?;

        if !results.status.success() {
            return Err(Error::other(format!("Had problems updating {}", package)));
        }
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

fn update_all(packages: &[&'static str], parallelism: bool, force: bool) -> Result<(), Error> {
    for package in packages {
        println!("Updating {}", package);
        update_package(package, parallelism, force)?;
    }
    Ok(())
}

pub fn handler(args: &ArgMatches) -> Result<(), Error> {
    let valid_args = ["cs2", "epiclang", "banana"];

    let parallelism = args.get_flag("parallelism");
    let force = args.get_flag("force");

    // TODO: remove this temporary solution to update all
    // without args and without -f or -j
    if !args.args_present() {
        return update_all(&valid_args, parallelism, force);
    }

    let mut found_args = false;
    for valid_arg in valid_args {
        if args.get_flag(valid_arg) {
            println!("Updating only {}", valid_arg);
            update_package(valid_arg, parallelism, force)?;
            found_args = true;
        };
    }

    if !found_args {
        return update_all(&valid_args, parallelism, force);
    }

    Ok(())
}
