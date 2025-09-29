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

fn update_package(package: &str, parallelism: bool) -> Result<(), Error> {
    let path = shared::get_final_path(package);

    if !Path::new(&path).exists() {
        return Err(Error::other(format!(
            "Impossible to find {}, have you installed it with cs2 install?",
            package
        )));
    }

    if pull_repo(&path, package)? {
        shared::build_package(package, parallelism)?;
    } else {
        println!("Nothing to update");
    }

    Ok(())
}

pub fn handler(args: &ArgMatches) -> Result<(), Error> {
    let parallelism = *args.get_one::<bool>("parallelism").unwrap();
    let valid_args = ["cs2", "epiclang", "banana"];

    if !args.args_present() {
        for arg in valid_args {
            update_package(arg, parallelism)?;
        }
        return Ok(());
    }

    for valid_arg in valid_args {
        if *args.get_one::<bool>(valid_arg).unwrap() {
            println!("Updating only {}", valid_arg);
            update_package(valid_arg, parallelism)?;
        };
    }

    Ok(())
}
