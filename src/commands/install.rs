use std::path::Path;
use std::process::Command;

const EPICLANG_REPO: &str = "git@github.com:Epitech/epiclang.git";
const BANANA_REPO: &str = "git@github.com:Epitech/banana-coding-style-checker.git";

fn fetch_repo(link: &str, path: &Path) -> bool {
    // TODO: raising errors here
    // TODO: replace
    let temp_path = "/tmp/cs2-temp-12";

    let final_path = match path.to_str() {
        Some(p) => p,
        None => return false,
    };

    let outputs = match Command::new("git")
        .args(["clone", link, temp_path])
        .output()
    {
        Ok(result) => result,
        Err(_) => return false,
    };

    if !outputs.status.success() {
        return false;
    }

    println!("{}", String::from_utf8(outputs.stderr).unwrap());

    return true;
}

fn epiclang() -> bool {
    if !fetch_repo(EPICLANG_REPO, Path::new("/usr/local/share/cs2/epiclang")) {
        println!("nuhuh");
        return false;
    }
    return true;
}

fn banana() -> bool {
    if !fetch_repo(BANANA_REPO, Path::new("/usr/local/share/cs2/banana")) {
        println!("nuhuh");
        return false;
    }
    return true;
}

pub fn all() -> bool {
    epiclang();
    banana();

    return true;
}
