use std::path::Path;
use std::process::Command;

const EPICLANG_REPO: &str = "git@github.com:Epitech/epiclang.git";
const BANANA_REPO: &str = "git@github.com:Epitech/banana-coding-style-checker.git";

// TODO: replace all bools by Result
// TODO: add logging

fn get_temp_path(package: &str) -> String {
    format!("/tmp/cs2-{}", package)
}

fn get_final_path(package: &str) -> String {
    format!("/usr/local/share/cs2/{}", package)
}

fn pull_repo(link: &str, temp_path: &str) -> bool {
    let clone_command = match Command::new("git")
        .args(["clone", link, temp_path])
        .status()
    {
        Ok(status) => status,
        Err(_) => return false,
    };

    if !clone_command.success() {
        return false;
    }
    return true;
}

fn move_to_final_path(temp_path: &str, final_path: &Path) -> bool {
    let final_path_str = match final_path.to_str() {
        Some(p) => p,
        None => return false,
    };

    if final_path.exists() {
        return false;
    }

    match Command::new("sudo")
        .args(["mv", temp_path, final_path_str])
        .status()
    {
        Ok(status) => {
            if !status.success() {
                return false;
            }
        }
        Err(_) => return false,
    };
    return true;
}

/// if clang-20 doesn't exist, check that clang installed version is `> 20`
/// if it is, create symlink for clang-20 in `/usr/local/bin`
fn verify_clang_version() -> bool {
    if Path::new("/usr/bin/clang-20").exists() {
        return true;
    };

    let version_output = match Command::new("clang").args(["--version"]).output() {
        Ok(s) => s,
        Err(_) => return false,
    };
    if !version_output.status.success() {
        return false;
    }

    let version_string = match String::from_utf8(version_output.stdout)
        .unwrap()
        .split(" ")
        .nth(2)
    {
        Some(v) => v.to_string(),
        None => return false,
    };

    let major: i32 = version_string.split(".").next().unwrap().parse().unwrap();
    if major > 20 {
        match Command::new("sudo")
            .args(["ln", "-s", "/usr/bin/clang", "/usr/local/bin/clang-20"])
            .status()
        {
            Ok(s) => s,
            Err(_) => return false,
        };
        return true;
    }

    return false;
}

fn epiclang() -> bool {
    let package = "epiclang";
    let temp_path = get_temp_path(package);

    if !pull_repo(EPICLANG_REPO, temp_path.as_str()) {
        return false;
    }

    match Command::new("chmod")
        .args(["+x", format!("{}/manual-install.sh", temp_path).as_str()])
        .status()
    {
        Ok(status) => {
            if !status.success() {
                return false;
            }
        }
        Err(_) => return false,
    };

    move_to_final_path(
        temp_path.as_str(),
        Path::new(&get_final_path(package)),
    );

    match Command::new("sh")
        .args([
            "-c",
            "cd /usr/local/share/cs2/epiclang && sudo ./manual-install.sh",
        ])
        .status()
    {
        Ok(status) => {
            if !status.success() {
                return false;
            }
        }
        Err(_) => return false,
    };
    return true;
}

fn banana() -> bool {
    let package = "banana";
    let temp_path = get_temp_path(package);

    if !pull_repo(BANANA_REPO, &temp_path) {
        return false;
    }
    return true;
}

fn create_directory() -> bool {
    match Command::new("sudo")
        .args(["mkdir", "-p", "/usr/local/share/cs2"])
        .status()
    {
        Ok(s) => {
            return s.success();
        }
        Err(_) => {}
    };
    return false;
}

pub fn all() -> bool {
    if !create_directory() {
        return false;
    }

    if !verify_clang_version() {
        return false;
    };

    if !epiclang() || !banana() {
        return false;
    };

    return true;
}
