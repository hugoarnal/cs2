use std::io::Error;
use std::path::Path;
use std::process::Command;

use clap::ArgMatches;

const EPICLANG_REPO: &str = "git@github.com:Epitech/epiclang.git";
const BANANA_REPO: &str = "git@github.com:Epitech/banana-coding-style-checker.git";

fn get_temp_path(package: &str) -> String {
    format!("/tmp/cs2-{}", package)
}

fn get_final_path(package: &str) -> String {
    format!("/usr/local/share/cs2/{}", package)
}

fn pull_repo(link: &str, temp_path: &str) -> Result<(), Error> {
    let clone_command = match Command::new("git")
        .args(["clone", link, temp_path])
        .status()
    {
        Ok(status) => status,
        Err(e) => return Err(e),
    };

    if !clone_command.success() {
        return Err(Error::other(format!(
            "Impossible to clone {}, make sure you have the permissions to do so.",
            link
        )));
    }
    return Ok(());
}

fn move_to_final_path(temp_path: &str, final_path: &Path) -> Result<(), Error> {
    let final_path_str = final_path.to_str().unwrap();

    if final_path.exists() {
        return Ok(());
    }

    match Command::new("sudo")
        .args(["mv", temp_path, final_path_str])
        .status()
    {
        Ok(status) => {
            if !status.success() {
                return Err(Error::other(format!(
                    "Impossible to move to {}",
                    final_path_str
                )));
            }
        }
        Err(e) => return Err(e),
    };
    return Ok(());
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

    let version_output = match Command::new("clang").args(["--version"]).output() {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
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
        match Command::new("sudo")
            .args(["ln", "-s", "/usr/bin/clang", "/usr/local/bin/clang-20"])
            .status()
        {
            Ok(s) => s,
            Err(e) => return Err(e),
        };
        return Ok(());
    }

    return Err(Error::other("clang version is not >= 20"));
}

fn epiclang() -> Result<(), Error> {
    let package = "epiclang";
    let temp_path = get_temp_path(package);
    let final_path = get_final_path(package);

    if Path::new(&final_path).exists() {
        return Err(Error::other(
            "Already cloned and installed, use cs2 update instead.",
        ));
    }

    match pull_repo(EPICLANG_REPO, temp_path.as_str()) {
        Ok(_) => {}
        Err(e) => return Err(e),
    }

    match Command::new("chmod")
        .args(["+x", format!("{}/manual-install.sh", temp_path).as_str()])
        .status()
    {
        Ok(status) => {
            if !status.success() {
                return Err(Error::other("Couldn't chmod manual-install.sh"));
            }
        }
        Err(e) => return Err(e),
    };

    match move_to_final_path(temp_path.as_str(), Path::new(&final_path)) {
        Ok(_) => {}
        Err(e) => return Err(e),
    };

    let build_command = format!("cd {} && sudo ./manual-install.sh", final_path);

    match Command::new("sh")
        .args(["-c", build_command.as_str()])
        .status()
    {
        Ok(status) => {
            if !status.success() {
                return Err(Error::other("Impossible to install epiclang"));
            }
        }
        Err(e) => return Err(e),
    };

    return Ok(());
}

fn banana() -> Result<(), Error> {
    let package = "banana";
    let temp_path = get_temp_path(package);
    let final_path = get_final_path(package);

    if Path::new(&final_path).exists() {
        return Err(Error::other(
            "Already cloned and installed, use cs2 update instead.",
        ));
    }

    match pull_repo(BANANA_REPO, temp_path.as_str()) {
        Ok(_) => {}
        Err(e) => return Err(e),
    }

    match move_to_final_path(temp_path.as_str(), Path::new(&final_path)) {
        Ok(_) => {}
        Err(e) => return Err(e),
    };

    let build_command = format!("cd {} && ./scripts/make_plugin.sh", final_path);

    match Command::new("sh")
        .args(["-c", build_command.as_str()])
        .status()
    {
        Ok(status) => {
            if !status.success() {
                return Err(Error::other("Impossible to build banana"));
            }
        }
        Err(e) => return Err(e),
    };

    match Command::new("sudo")
        .args([
            "mv",
            format!("{}/epiclang-plugin-banana.so", final_path).as_str(),
            "/usr/local/lib/epiclang/plugins/epiclang-plugin-banana.so",
        ])
        .status()
    {
        Ok(status) => {
            if !status.success() {
                return Err(Error::other(
                    "Impossible to move banana plugin to the plugin directory",
                ));
            }
        }
        Err(e) => return Err(e),
    };

    return Ok(());
}

fn create_directory() -> Result<(), Error> {
    let path = "/usr/local/share/cs2";

    if Path::new(path).exists() {
        return Ok(());
    };

    match Command::new("sudo").args(["mkdir", "-p", path]).status() {
        Ok(_) => {
            return Ok(());
        }
        Err(e) => return Err(e),
    };
}

fn verify() -> Result<(), Error> {
    match create_directory() {
        Ok(_) => {}
        Err(e) => return Err(e),
    }

    match verify_clang_version() {
        Ok(_) => {}
        Err(e) => return Err(e),
    };

    Ok(())
}

pub fn all() -> Result<(), Error> {
    match epiclang() {
        Ok(_) => {}
        Err(e) => return Err(e),
    };

    match banana() {
        Ok(_) => {}
        Err(e) => return Err(e),
    };

    return Ok(());
}

pub fn handler(args: &ArgMatches) -> Result<(), Error> {
    verify()?;

    if *args.get_one::<bool>("epiclang").unwrap() {
        println!("Installing only epiclang");
        epiclang()?;
    };
    if *args.get_one::<bool>("banana").unwrap() {
        println!("Installing only banana");
        banana()?;
    };

    if !args.args_present() {
        all()?;
    };

    Ok(())
}
