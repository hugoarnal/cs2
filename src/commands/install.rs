use std::io::Error;
use std::path::Path;
use std::process::Command;

use clap::ArgMatches;

use crate::commands::shared;

const EPICLANG_REPO: &str = "git@github.com:Epitech/epiclang.git";
const BANANA_REPO: &str = "git@github.com:Epitech/banana-coding-style-checker.git";

fn pull_repo(link: &str, temp_path: &str) -> Result<(), Error> {
    if !Command::new("git")
        .args(["clone", link, temp_path])
        .status()?
        .success()
    {
        return Err(Error::other(format!(
            "Impossible to clone {}, make sure you have the permissions to do so",
            link
        )));
    };

    return Ok(());
}

fn move_to_final_path(temp_path: &str, final_path: &Path) -> Result<(), Error> {
    let final_path_str = final_path.to_str().unwrap();

    if final_path.exists() {
        return Ok(());
    }

    if !Command::new("sudo")
        .args(["mv", temp_path, final_path_str])
        .status()?
        .success()
    {
        return Err(Error::other(format!(
            "Impossible to move to {}",
            final_path_str
        )));
    };
    Ok(())
}

/// if clang-20 doesn't exist, check that clang installed version is `> 20`
/// if it is, create symlink for clang-20 in `/usr/local/bin`
fn verify_clang_version() -> Result<(), Error> {
    if Path::new("/usr/bin/clang-20").exists() {
        return Ok(());
    };

    if !Path::new("/usr/bin/clang").exists() {
        return Err(Error::other("Impossible to find clang, is it installed?"));
    };

    let version_output = Command::new("clang").args(["--version"]).output()?;
    if !version_output.status.success() {
        return Err(Error::other("Impossible to get clang version"));
    }

    let version_string = match String::from_utf8(version_output.stdout)
        .unwrap()
        .split(" ")
        .nth(2)
    {
        Some(v) => v.to_string(),
        None => return Err(Error::other("Impossible to get clang version")),
    };

    let major: i32 = version_string.split(".").next().unwrap().parse().unwrap();
    if major > 20 {
        Command::new("sudo")
            .args(["ln", "-s", "/usr/bin/clang", "/usr/local/bin/clang-20"])
            .spawn()?;
    }

    return Err(Error::other("clang version is not >= 20"));
}

fn epiclang() -> Result<(), Error> {
    let package = "epiclang";
    let temp_path = shared::get_temp_path(package);
    let final_path = shared::get_final_path(package);

    if Path::new(&final_path).exists() {
        return Err(Error::other(
            "Already cloned and installed, use cs2 update instead",
        ));
    }

    pull_repo(EPICLANG_REPO, temp_path.as_str())?;

    if !Command::new("chmod")
        .args(["+x", format!("{}/manual-install.sh", temp_path).as_str()])
        .status()?
        .success()
    {
        return Err(Error::other("Couldn't chmod manual-install.sh"));
    }

    move_to_final_path(temp_path.as_str(), Path::new(&final_path))?;

    shared::build_epiclang(&final_path)?;

    return Ok(());
}

fn banana(parallelism: bool) -> Result<(), Error> {
    let package = "banana";
    let temp_path = shared::get_temp_path(package);
    let final_path = shared::get_final_path(package);

    if Path::new(&final_path).exists() {
        return Err(Error::other(
            "Already cloned and installed, use cs2 update instead",
        ));
    }

    pull_repo(BANANA_REPO, temp_path.as_str())?;

    move_to_final_path(temp_path.as_str(), Path::new(&final_path))?;

    shared::build_banana(&final_path, parallelism)?;

    return Ok(());
}

fn create_directory() -> Result<(), Error> {
    let path = shared::get_final_path("");

    if Path::new(&path).exists() {
        return Ok(());
    };

    match Command::new("sudo").args(["mkdir", "-p", &path]).status() {
        Ok(_) => {
            return Ok(());
        }
        Err(e) => return Err(e),
    };
}

pub fn handler(args: &ArgMatches) -> Result<(), Error> {
    let parallelism = *args.get_one::<bool>("parallelism").unwrap();

    create_directory()?;
    verify_clang_version()?;

    if *args.get_one::<bool>("epiclang").unwrap() {
        println!("Installing only epiclang");
        epiclang()?;
    };
    if *args.get_one::<bool>("banana").unwrap() {
        println!("Installing only banana");
        banana(parallelism)?;
    };

    if !args.args_present() {
        epiclang()?;
        banana(parallelism)?;
    };

    Ok(())
}
