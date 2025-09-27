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

fn update_package(package: &str) -> Result<(), Error> {
    let path = shared::get_final_path(package);

    if !Path::new(&path).exists() {
        return Err(Error::other(format!(
            "Impossible to find {}, have you installed it with cs2 install?",
            package
        )));
    }

    if pull_repo(&path, package)? {
        shared::build_package(package)?;
    } else {
        println!("Nothing to update");
    }

    Ok(())
}

pub fn handler(args: &ArgMatches) -> Result<(), Error> {
    if *args.get_one::<bool>("epiclang").unwrap() {
        println!("Updating only epiclang");
        update_package("epiclang")?;
    };
    if *args.get_one::<bool>("banana").unwrap() {
        println!("Updating only banana");
        update_package("banana")?;
    };

    if !args.args_present() {
        update_package("epiclang")?;
        update_package("banana")?;
    };

    Ok(())
}
