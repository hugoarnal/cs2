use std::path::Path;
use std::process::Command;
use std::str::FromStr;

use anyhow::Result;
use thiserror::Error;

use crate::{
    commands::shared::{get_final_path, warn_path_var},
    package::Packages,
    shared::create_directory,
};

#[derive(Error, Debug)]
enum InstallError {
    #[error("Impossible to find clang, are you sure it's installed?")]
    CantFindClang,

    #[error("Impossible to find clang++")]
    CantFindClangPP,

    #[error("Impossible to get clang version")]
    CantGetClangVersion,

    #[error("Incorrect version (clang version is not >= 20)")]
    IncorrectClangVersion,
}

/// if clang-20 doesn't exist, check that clang installed version is `> 20`
/// if it is, create symlink for clang-20 in `/usr/local/bin`
fn verify_clang_version() -> Result<()> {
    let possible_paths = ["/usr/bin", "/usr/local/bin"];

    for path in possible_paths {
        if Path::new(&format!("{}/clang-20", path)).exists() {
            return Ok(());
        };
    }

    if !Path::new("/usr/bin/clang").exists() {
        return Err(InstallError::CantFindClang.into());
    };

    let version_output = Command::new("clang").args(["--version"]).output()?;
    if !version_output.status.success() {
        return Err(InstallError::CantGetClangVersion.into());
    }

    let version_string = match String::from_utf8(version_output.stdout)?
        .split("version ")
        .nth(1)
    {
        Some(v) => v.to_string(),
        None => return Err(InstallError::CantGetClangVersion.into()),
    };

    let major: i32 = match version_string.split(".").next() {
        Some(s) => s.parse()?,
        None => return Err(InstallError::CantGetClangVersion.into()),
    };

    if major >= 20 {
        let _ = Command::new("sudo")
            .args(["ln", "-s", "/usr/bin/clang", "/usr/local/bin/clang-20"])
            .spawn()?
            .wait();

        _ = warn_path_var("/usr/local/bin");

        return Ok(());
    }

    Err(InstallError::IncorrectClangVersion.into())
}

fn verify_clangpp_version() -> Result<()> {
    if !Path::new("/usr/bin/clang++").exists() {
        println!("clang++ doesn't exist");
        return Err(InstallError::CantFindClangPP.into());
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

fn install_all(parallelism: &String) -> Result<()> {
    let all_packages = [Packages::Epiclang, Packages::Banana];

    for package in all_packages {
        package.install(parallelism)?;
    }
    Ok(())
}

pub fn handler(package: &Option<String>, jobs: &String) -> Result<()> {
    create_directory(get_final_path("").as_str())?;
    verify_clang_version()?;
    verify_clangpp_version()?;

    if let Some(package_str) = package {
        let package = Packages::from_str(package_str)?;
        return package.install(jobs);
    }

    install_all(jobs)
}
