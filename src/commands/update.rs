use anyhow::{anyhow, Result};
use std::{path::Path, process::Command, str::FromStr};

use crate::package::Packages;

/// Returns true if project needs to be rebuilt, false if it's already at the latest version
pub fn pull_repo(path: &str, package: &str) -> Result<bool> {
    let command = format!("cd {} && git pull origin main", path);
    let results = Command::new("sh").args(["-c", &command]).output()?;

    if !results.status.success() {
        let command = format!(
            "cd {} && git reset --hard main && git pull origin main",
            path
        );
        let results = Command::new("sh").args(["-c", &command]).output()?;

        if !results.status.success() {
            return Err(anyhow!(
                "Had problems updating {}: {}",
                package,
                String::from_utf8(results.stderr)?
            ));
        }
    };

    if String::from_utf8(results.stdout)?.contains("Already up to date.") {
        Ok(false)
    } else {
        Ok(true)
    }
}

fn update_all(parallelism: &String, force: bool) -> Result<()> {
    let packages = [Packages::Cs2, Packages::Epiclang, Packages::Banana];

    for package in packages {
        if let Err(e) = package.update(parallelism, force) {
            println!("{}", e);
        };
    }
    Ok(())
}

/// It's there for future updates and especially the depreciation of
/// `banana-check-repo-cs2`
/// Does cleanup work, checks if there are files that shouldn't be there,
/// or should be moved and such.
/// Doesn't actually remove them for you, but suggests that they can be removed.
fn pre_update() -> Result<()> {
    if Path::new("/usr/local/bin/banana-check-repo-cs2").exists() {
        println!("cs2 no longer uses /usr/local/bin/banana-check-repo-cs2");
        println!("You can safely remove this file from your computer with:");
        println!("$ sudo rm /usr/local/bin/banana-check-repo-cs2 (this wasn't ran, it's up to you to do it.)");
    }
    Ok(())
}

pub fn handler(package: &Option<String>, jobs: &String, force: bool) -> Result<()> {
    pre_update()?;

    if let Some(package_str) = package {
        let package = Packages::from_str(package_str)?;
        return package.update(jobs, force);
    }

    update_all(jobs, force)
}
