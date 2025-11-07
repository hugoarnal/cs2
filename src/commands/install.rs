use std::path::Path;
use std::process::Command;
use std::{io::Error, str::FromStr};

use crate::{
    commands::shared::{get_final_path, warn_path_var},
    package::Packages,
    shared::create_directory,
};

/// if clang-20 doesn't exist, check that clang installed version is `> 20`
/// if it is, create symlink for clang-20 in `/usr/local/bin`
fn verify_clang_version() -> Result<(), Error> {
    let possible_paths = ["/usr/bin", "/usr/local/bin"];

    for path in possible_paths {
        if Path::new(&format!("{}/clang-20", path)).exists() {
            return Ok(());
        };
    }

    if !Path::new("/usr/bin/clang").exists() {
        return Err(Error::other("Impossible to find clang, is it installed?"));
    };

    let version_output = Command::new("clang").args(["--version"]).output()?;
    if !version_output.status.success() {
        return Err(Error::other("Impossible to get clang version"));
    }

    let version_string = match String::from_utf8(version_output.stdout)
        .unwrap()
        .split("version ")
        .nth(1)
    {
        Some(v) => v.to_string(),
        None => return Err(Error::other("Impossible to get clang version")),
    };

    let major: i32 = match version_string.split(".").next() {
        Some(s) => s.parse().unwrap(),
        None => return Err(Error::other("Impossible to get the clang major version")),
    };

    if major >= 20 {
        let _ = Command::new("sudo")
            .args(["ln", "-s", "/usr/bin/clang", "/usr/local/bin/clang-20"])
            .spawn()?
            .wait();

        warn_path_var("/usr/local/bin");

        return Ok(());
    }

    Err(Error::other("clang version is not >= 20"))
}

fn verify_clangpp_version() -> Result<(), Error> {
    if !Path::new("/usr/bin/clang++").exists() {
        println!("clang++ doesn't exist");
        return Err(Error::other("Impossible to find clang++"));
    }

    if Path::new("/usr/local/bin/clang++-20").exists() {
        return Ok(());
    }

    // Assume that clang++ version is the same as clang (there's no reason it isn't)
    let _ = Command::new("sudo")
        .args(["ln", "-s", "/usr/bin/clang++", "/usr/local/bin/clang++-20"])
        .spawn()?
        .wait();

    Ok(())
}

fn install_all(parallelism: &String) -> Result<(), Error> {
    let all_packages = [Packages::Epiclang, Packages::Banana];

    for package in all_packages {
        if let Err(e) = package.install(parallelism) {
            println!("{}", e);
        };
    }
    Ok(())
}

pub fn handler(package: &Option<String>, jobs: &String) -> Result<(), Error> {
    create_directory(get_final_path("").as_str())?;
    verify_clang_version()?;
    verify_clangpp_version()?;

    if let Some(package_str) = package {
        let package = Packages::from_str(package_str)?;
        return package.install(jobs);
    }

    install_all(jobs)
}
