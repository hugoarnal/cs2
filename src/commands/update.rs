use anyhow::Result;
use std::path::Path;

use crate::{
    commands::shared::get_final_path,
    packages::{self, source::pull_repo, Package, PackageType},
};

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

fn update_package(package: &mut Box<dyn Package>, jobs: &str, force: bool) -> Result<()> {
    let package_name = package.as_str();
    package.set_parallelism(jobs);

    if package.get_type() == PackageType::Source {
        let path = get_final_path(package_name);
        println!("Updating {}", package_name);

        if pull_repo(&path, package_name)? || force {
            package.build()?;
        } else {
            println!("Nothing to update");
            return Ok(());
        }
    } else {
        println!("Updating {}", package_name);
        package.download()?;
        package.build()?;
        package.install()?;
    }

    println!("Successfully updated {}", package_name);
    Ok(())
}

pub fn handler(package: &Option<String>, jobs: &str, force: bool) -> Result<()> {
    pre_update()?;

    if let Some(package_str) = package {
        let mut package = packages::from_str(package_str)?;
        return update_package(&mut package, jobs, force);
    }

    // TODO: update all installed packages by default
    Ok(())
}
