use std::path::Path;
use std::process::Command;

const EPICLANG_REPO: &str = "git@github.com:Epitech/epiclang.git";
const BANANA_REPO: &str = "git@github.com:Epitech/banana-coding-style-checker.git";

// TODO: replace all bools by Result

fn pull_repo(program: &str, link: &str, path: &Path) -> bool {
    let temp_path = format!("/tmp/cs2-{}", program);

    let final_path = match path.to_str() {
        Some(p) => p,
        None => return false,
    };

    let clone_command = match Command::new("git")
        .args(["clone", link, temp_path.as_str()])
        .status()
    {
        Ok(status) => status,
        Err(_) => return false,
    };

    if !clone_command.success() || path.exists() {
        return false;
    }

    match Command::new("sudo")
        .args(["mv", temp_path.as_str(), final_path])
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
    if !pull_repo(
        "epiclang",
        EPICLANG_REPO,
        Path::new("/usr/local/share/cs2/epiclang"),
    ) {
        return false;
    }
    return true;
}

fn banana() -> bool {
    if !pull_repo(
        "banana",
        BANANA_REPO,
        Path::new("/usr/local/share/cs2/banana"),
    ) {
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
