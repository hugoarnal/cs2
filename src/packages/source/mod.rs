pub mod banana;
pub mod epiclang;

use std::process::Command;

use anyhow::{anyhow, Result};

fn clone_repo(link: &str, temp_path: &str) -> Result<()> {
    if !Command::new("git")
        .args(["clone", link, temp_path])
        .status()?
        .success()
    {
        return Err(anyhow!(
            "Impossible to clone {}, make sure you have the permissions to do so",
            link
        ));
    };

    Ok(())
}

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
