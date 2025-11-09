use std::fmt;
use std::path::Path;
use std::process::Command;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use thiserror::Error;

use crate::commands::{
    shared::{get_final_path, get_temp_path, warn_path_var},
    update::pull_repo,
};

const EPICLANG_REPO: &str = "git@github.com:Epitech/epiclang.git";
const BANANA_REPO: &str = "git@github.com:Epitech/banana-coding-style-checker.git";

#[derive(Clone, Debug, PartialEq)]
pub enum Packages {
    Cs2,
    Epiclang,
    Banana,
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

impl FromStr for Packages {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self> {
        match input.to_ascii_lowercase().as_str() {
            "cs2" => Ok(Self::Cs2),
            "epiclang" => Ok(Self::Epiclang),
            "banana" => Ok(Self::Banana),
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
            Self::Banana => "banana",
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
                    "cd {} && sudo ./manual-install.sh",
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
            Self::Banana => &[
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

                if !Command::new("chmod")
                    .args(["+x", format!("{}/manual-install.sh", temp_path).as_str()])
                    .status()?
                    .success()
                {
                    return Err(anyhow!("Couldn't chmod manual-install.sh"));
                }

                move_to_final_path(temp_path.as_str(), Path::new(&final_path))?;
            }
            Self::Banana => {
                clone_repo(BANANA_REPO, temp_path.as_str())?;
                move_to_final_path(temp_path.as_str(), Path::new(&final_path))?;
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
