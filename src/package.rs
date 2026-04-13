use std::fmt;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::str::FromStr;

#[allow(unused_imports)]
use crate::patches;
use crate::shared::download_file;
use anyhow::{anyhow, Result};
use regex::Regex;
use thiserror::Error;

use crate::commands::{
    shared::{get_final_path, get_temp_path, warn_path_var},
    update::pull_repo,
};

const EPICLANG_REPO: &str = "git@github.com:Epitech/epiclang.git";
const BANANA_REPO: &str = "git@github.com:Epitech/banana-coding-style-checker.git";

// Keep the slash at the end or you get a 301 on request
const BANANA_PPA_LINK: &str =
    "https://ppa.launchpadcontent.net/epitech/ppa/ubuntu/pool/main/b/banana-coding-style-checker/";
const EPICLANG_PPA_LINK: &str =
    "https://ppa.launchpadcontent.net/epitech/ppa/ubuntu/pool/main/e/epiclang/";
const TAR_XZ_PPA_REGEX: &str = r"<a[^>]+>(.+?\.tar\.xz)</a>";
const BANANA_FINAL_TAR_FILE: &str = "/tmp/banana.tar.xz";
const EPICLANG_FINAL_TAR_FILE: &str = "/tmp/epiclang.tar.xz";

#[derive(Clone, Debug, PartialEq)]
pub enum Packages {
    Cs2,
    Epiclang,
    EpiclangBinary,
    Banana,
    BananaBinary,
    BananaCheckRepo,
}

#[derive(Error, Debug)]
enum PackagesError {
    #[error("Impossible to build {0}")]
    Build(Packages),

    #[error("Impossible to install {0}")]
    Install(Packages),

    #[error("Impossible to move {0} to it's destination")]
    Move(Packages),

    #[error("Impossible to find {0}, are you sure it is installed?")]
    NotFound(Packages),

    #[error("Impossible to clone {0}, make sure you have the permissions to do so")]
    RepoClone(String),

    #[error("Already installed, use cs2 update instead")]
    AlreadyInstalled,

    #[error(
        "{0} seems to be installed by a package manager, cs2 won't be able to install/update it"
    )]
    InstalledByPackageManager(String),
}

fn clone_repo(link: &str, temp_path: &str) -> Result<()> {
    if !Command::new("git")
        .args(["clone", link, temp_path])
        .status()?
        .success()
    {
        return Err(PackagesError::RepoClone(link.to_string()).into());
    };

    Ok(())
}

fn move_to_final_path(temp_path: &str, final_path: &Path) -> Result<()> {
    let final_path_str = final_path.to_str().unwrap();

    if final_path.exists() {
        return Ok(());
    }

    if !Command::new("sudo")
        .args(["mv", temp_path, final_path_str])
        .status()?
        .success()
    {
        return Err(anyhow!("Impossible to move to {}", final_path_str));
    };
    Ok(())
}

fn download_html_ppa(link: &str, file: &str, final_file: &str) -> Result<(), anyhow::Error> {
    download_file(link, file)?;

    let tar_xz_file: String;

    match fs::read_to_string(file) {
        Ok(content) => {
            let re = Regex::new(TAR_XZ_PPA_REGEX);

            if let Some((_, [file])) = re
                .expect("REASON")
                .captures_iter(&content)
                .map(|c| c.extract())
                .next()
            {
                tar_xz_file = String::from(file);
            } else {
                return Err(anyhow!("Impossible to find tar"));
            }
        }
        Err(_) => {
            return Err(anyhow!("Couldn't get the result of {}", file));
        }
    }

    download_file(format!("{}/{}", link, tar_xz_file).as_str(), final_file)?;
    Ok(())
}

fn untar_ppa(temp_dir: &str, final_tar: &str) -> Result<(), anyhow::Error> {
    if !Command::new("mkdir")
        .args(["-p", temp_dir])
        .status()?
        .success()
    {
        return Err(anyhow!("Couldn't create dir {}", temp_dir));
    }

    if !Command::new("tar")
        .args(["xf", final_tar, "--strip-components=1", "-C", temp_dir])
        .status()?
        .success()
    {
        return Err(anyhow!("Couldn't untar {}", final_tar));
    }
    Ok(())
}

impl FromStr for Packages {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self> {
        match input.to_ascii_lowercase().as_str() {
            "cs2" => Ok(Self::Cs2),
            "epiclang" => Ok(Self::Epiclang),
            "epiclang-bin" => Ok(Self::EpiclangBinary),
            "banana" => Ok(Self::Banana),
            "banana-bin" => Ok(Self::BananaBinary),
            "banana-check-repo" => Ok(Self::BananaCheckRepo),
            _ => Err(anyhow!("Couldn't find package")),
        }
    }
}

impl Packages {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Self::Cs2 => "cs2",
            Self::Epiclang => "epiclang",
            Self::EpiclangBinary => "epiclang-bin",
            Self::Banana => "banana",
            Self::BananaBinary => "banana-bin",
            Self::BananaCheckRepo => "banana-check-repo",
        }
    }

    pub fn build(&self, parallelism: &String) -> Result<()> {
        match *self {
            Self::Cs2 => {
                let build_command = format!("cd {} && ./compile.sh", get_final_path(self.as_str()));

                if !Command::new("sh")
                    .args(["-c", build_command.as_str()])
                    .status()?
                    .success()
                {
                    return Err(PackagesError::Build(Self::Cs2).into());
                }
            }
            Self::Epiclang => {
                let build_command = format!(
                    "cd {} && sudo sh ./manual-install.sh",
                    get_final_path(self.as_str())
                );

                if !Command::new("sh")
                    .args(["-c", build_command.as_str()])
                    .status()?
                    .success()
                {
                    return Err(PackagesError::Install(Self::Epiclang).into());
                }
            }
            Self::EpiclangBinary => {
                const TEMP_EPICLANG_DIR: &str = "/tmp/epiclang-binary";
                untar_ppa(TEMP_EPICLANG_DIR, EPICLANG_FINAL_TAR_FILE)?;

                let install_command = format!(
                    "sudo install -Dm755 {}/install/0/epiclang.py /usr/local/bin/epiclang.py && sudo install -Dm755 {}/install/0/epiclang /usr/local/bin/epiclang",
                    TEMP_EPICLANG_DIR, TEMP_EPICLANG_DIR
                );

                if !Command::new("bash")
                    .args(["-c", install_command.as_str()])
                    .status()?
                    .success()
                {
                    return Err(anyhow!(
                        "Couldn't install epiclang {}",
                        EPICLANG_FINAL_TAR_FILE
                    ));
                }
            }
            Self::Banana => {
                let final_path = get_final_path(self.as_str());
                let build_command = format!("cd {} && ./scripts/make_plugin.sh", final_path);

                let mut full_command = Command::new("sh");
                full_command.args(["-c", build_command.as_str()]);

                full_command.env("CMAKE_BUILD_PARALLEL_LEVEL", parallelism);

                if !full_command.status()?.success() {
                    return Err(PackagesError::Build(Self::Banana).into());
                }

                if !Command::new("sudo")
                    .args([
                        "install",
                        "-Dm755",
                        format!("{}/epiclang-plugin-banana.so", final_path).as_str(),
                        "/usr/local/lib/epiclang/plugins/epiclang-plugin-banana.so",
                    ])
                    .status()?
                    .success()
                {
                    return Err(PackagesError::Install(Self::Banana).into());
                }

                // checks that banana-check-repo is installed or "builds" it if it isn't
                if Packages::BananaCheckRepo.verify_install().is_ok() {
                    Packages::BananaCheckRepo.build(parallelism)?;
                }
            }
            Self::BananaBinary => {
                const TEMP_BANANA_DIR: &str = "/tmp/banana-binary";
                untar_ppa(TEMP_BANANA_DIR, BANANA_FINAL_TAR_FILE)?;

                // TODO: yeah... can do better...
                let install_so_command = format!(
                    "sudo install -Dm755 `find {} -name epiclang-plugin-banana.so.* -type f` /usr/local/lib/epiclang/plugins/epiclang-plugin-banana.so",
                    TEMP_BANANA_DIR
                );

                if !Command::new("bash")
                    .args(["-c", install_so_command.as_str()])
                    .status()?
                    .success()
                {
                    return Err(anyhow!("Couldn't install library"));
                }

                // Overwriting the current banana-check-repo
                let install_check_repo_command = format!(
                    "sudo install -Dm755 {}/install/0/banana-check-repo /usr/local/bin/banana-check-repo",
                    TEMP_BANANA_DIR
                );

                if !Command::new("bash")
                    .args(["-c", install_check_repo_command.as_str()])
                    .status()?
                    .success()
                {
                    return Err(anyhow!(
                        "Couldn't install banana-check-repo {}",
                        BANANA_FINAL_TAR_FILE
                    ));
                }
            }
            Self::BananaCheckRepo => {
                let final_path = get_final_path("banana");
                if !Path::new(&final_path).exists() {
                    return Err(anyhow!(
                        "Impossible to find banana repo, are you sure it is installed?",
                    ));
                }

                let file_name = format!("{}/src/banana-check-repo", final_path);

                if !Command::new("sudo")
                    .args([
                        "install",
                        "-Dm755",
                        file_name.as_str(),
                        "/usr/local/bin/banana-check-repo",
                    ])
                    .status()?
                    .success()
                {
                    return Err(PackagesError::Move(Self::BananaCheckRepo).into());
                }
            }
        }
        Ok(())
    }

    pub fn get_packages(&self) -> &[&str] {
        match *self {
            Self::Epiclang => &["/usr/bin/epiclang", "/usr/local/bin/epiclang"],
            Self::Banana | Self::BananaBinary => &[
                "/usr/lib/epiclang/plugins/epitech-plugin-banana.so",
                "/usr/lib/epiclang/plugins/epiclang-plugin-banana.so",
                "/usr/local/lib/epiclang/plugins/epiclang-plugin-banana.so",
            ],
            Self::BananaCheckRepo => &[
                "/usr/bin/banana-check-repo",
                "/usr/local/bin/banana-check-repo",
            ],
            _ => &[],
        }
    }

    pub fn verify_install(&self) -> Result<()> {
        let packages = self.get_packages();
        let final_path = get_final_path(self.as_str());

        for package in packages {
            if Path::new(package).exists() && !Path::new(&final_path).exists() {
                return Err(PackagesError::InstalledByPackageManager(package.to_string()).into());
            }
        }
        Ok(())
    }

    pub fn install(&self, parallelism: &String) -> Result<()> {
        let package = self.as_str();
        let temp_path = get_temp_path(package);
        let final_path = get_final_path(package);

        self.verify_install()?;

        if Path::new(&final_path).exists() {
            return Err(PackagesError::AlreadyInstalled.into());
        }

        println!("Installing {}", package);

        match *self {
            Self::Epiclang => {
                clone_repo(EPICLANG_REPO, temp_path.as_str())?;
                move_to_final_path(temp_path.as_str(), Path::new(&final_path))?;
            }
            Self::Banana => {
                clone_repo(BANANA_REPO, temp_path.as_str())?;
                move_to_final_path(temp_path.as_str(), Path::new(&final_path))?;
            }
            Self::BananaBinary => {
                download_html_ppa(
                    BANANA_PPA_LINK,
                    "/tmp/banana-ppa-result.html",
                    BANANA_FINAL_TAR_FILE,
                )?;
            }
            Self::EpiclangBinary => {
                download_html_ppa(
                    EPICLANG_PPA_LINK,
                    "/tmp/epiclang-ppa-result.html",
                    EPICLANG_FINAL_TAR_FILE,
                )?;
            }
            _ => {}
        }

        self.build(parallelism)?;
        _ = warn_path_var("/usr/local/bin");

        Ok(())
    }

    pub fn update(&self, parallelism: &String, force: bool) -> Result<()> {
        let package = self.as_str();
        let path = get_final_path(package);

        self.verify_install()?;

        if !Path::new(&path).exists() {
            return Err(PackagesError::NotFound(self.clone()).into());
        }

        println!("Updating {}", package);

        if pull_repo(&path, self.as_str())? || force {
            self.build(parallelism)?;
        } else {
            println!("Nothing to update");
        }

        Ok(())
    }
}

impl fmt::Display for Packages {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
