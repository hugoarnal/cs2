use std::fmt;
use std::io::Error;
use std::path::Path;
use std::process::Command;
use std::str::FromStr;

use crate::commands::{
    shared::{get_final_path, get_temp_path, warn_path_var},
    update::pull_repo,
};
use crate::shared::create_directory;

const EPICLANG_REPO: &str = "git@github.com:Epitech/epiclang.git";
const BANANA_REPO: &str = "git@github.com:Epitech/banana-coding-style-checker.git";

pub enum Packages {
    Cs2,
    Epiclang,
    Banana,
    BananaCheckRepo,
    BananaCheckRepoCs2,
}

fn patch_file(patch_name: &str, file: &str) -> Result<(), Error> {
    let patch_path = format!("{}/src/patches/{}", get_final_path("cs2"), patch_name);

    if !Path::new(&patch_path).exists() {
        return Err(Error::other(
            "Impossible to find patch, try running cs2's installation script (install.sh) or compilation script (compile.sh)",
        ));
    }

    let command = Command::new("patch")
        .args(["-p0", "-s", "-f", file, &patch_path])
        .status()?;

    if !command.success() {
        println!("{} already applied to {}", patch_name, file);
        return Ok(());
    }

    println!("Applied {} to {}", patch_name, file);

    Ok(())
}

fn clone_repo(link: &str, temp_path: &str) -> Result<(), Error> {
    if !Command::new("git")
        .args(["clone", link, temp_path])
        .status()?
        .success()
    {
        return Err(Error::other(format!(
            "Impossible to clone {}, make sure you have the permissions to do so",
            link
        )));
    };

    return Ok(());
}

fn move_to_final_path(temp_path: &str, final_path: &Path) -> Result<(), Error> {
    let final_path_str = final_path.to_str().unwrap();

    if final_path.exists() {
        return Ok(());
    }

    if !Command::new("sudo")
        .args(["mv", temp_path, final_path_str])
        .status()?
        .success()
    {
        return Err(Error::other(format!(
            "Impossible to move to {}",
            final_path_str
        )));
    };
    Ok(())
}

impl FromStr for Packages {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Error> {
        match input.to_ascii_lowercase().as_str() {
            "cs2" => Ok(Self::Cs2),
            "epiclang" => Ok(Self::Epiclang),
            "banana" => Ok(Self::Banana),
            "banana-check-repo" => Ok(Self::BananaCheckRepo),
            "banana-check-repo-cs2" => Ok(Self::BananaCheckRepoCs2),
            _ => Err(Error::other("Couldn't find package")),
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
            Self::BananaCheckRepoCs2 => "banana-check-repo-cs2",
        }
    }

    pub fn build(&self, parallelism: bool) -> Result<(), Error> {
        match *self {
            Self::Cs2 => {
                let build_command = format!("cd {} && ./compile.sh", get_final_path(self.as_str()));

                if !Command::new("sh")
                    .args(["-c", build_command.as_str()])
                    .status()?
                    .success()
                {
                    return Err(Error::other("Impossible to build cs2"));
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
                    return Err(Error::other("Impossible to install epiclang"));
                }
            }
            Self::Banana => {
                let final_path = get_final_path(self.as_str());
                let build_command = format!("cd {} && ./scripts/make_plugin.sh", final_path);

                let mut full_command = Command::new("sh");
                full_command.args(["-c", build_command.as_str()]);

                if parallelism {
                    full_command.env(
                        "CMAKE_BUILD_PARALLEL_LEVEL",
                        std::thread::available_parallelism()?.get().to_string(),
                    );
                }

                if !full_command.status()?.success() {
                    return Err(Error::other("Impossible to build banana"));
                }

                let plugins_dir = "/usr/local/lib/epiclang/plugins";

                if !Path::new(plugins_dir).exists() {
                    create_directory(plugins_dir)?;
                }

                if !Command::new("sudo")
                    .args([
                        "cp",
                        format!("{}/epiclang-plugin-banana.so", final_path).as_str(),
                        "/usr/local/lib/epiclang/plugins/epiclang-plugin-banana.so",
                    ])
                    .status()?
                    .success()
                {
                    return Err(Error::other(
                        "Impossible to move banana plugin to the plugin directory",
                    ));
                }

                // checks that banana-check-repo is installed or "builds" it if it isn't
                // then builds banana-check-repo-cs2
                if Packages::BananaCheckRepo.verify_install().is_ok() {
                    Packages::BananaCheckRepo.build(parallelism)?;
                }
                Packages::BananaCheckRepoCs2.build(parallelism)?;
            }
            Self::BananaCheckRepo => {
                let final_path = get_final_path("banana");
                let file_name = format!("{}/src/banana-check-repo", final_path);

                if !Command::new("sudo")
                    .args(["cp", file_name.as_str(), "/usr/local/bin/banana-check-repo"])
                    .status()?
                    .success()
                {
                    return Err(Error::other("Impossible to move banana-check-repo"));
                }
            }
            Self::BananaCheckRepoCs2 => {
                let bcr_file_name = Packages::BananaCheckRepo
                    .get_installed_package()
                    .ok_or(Error::other("Impossible to find banana-check-repo"))?;
                let tmp_file_name = "/tmp/banana-check-repo-cs2";
                let file_name = "/usr/local/bin/banana-check-repo-cs2";

                if !Command::new("cp")
                    .args([bcr_file_name, tmp_file_name])
                    .status()?
                    .success()
                {
                    return Err(Error::other("Impossible to create banana-check-repo-cs2"));
                }

                patch_file("banana-check-repo-cs2.patch", tmp_file_name)?;

                if !Command::new("sudo")
                    .args(["cp", tmp_file_name, file_name])
                    .status()?
                    .success()
                {
                    return Err(Error::other("Impossible to install banana-check-repo-cs2"));
                }
            }
        }
        Ok(())
    }

    fn get_installed_package(&self) -> Option<&str> {
        for package in self.get_packages() {
            if Path::new(package).exists() {
                return Some(package);
            }
        }
        None
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
            Self::BananaCheckRepoCs2 => &["/usr/local/bin/banana-check-repo-cs2"],
            _ => &[],
        }
    }

    pub fn verify_install(&self) -> Result<(), Error> {
        let packages = self.get_packages();
        let final_path = get_final_path(self.as_str());

        for package in packages {
            if Path::new(package).exists() && !Path::new(&final_path).exists() {
                return Err(Error::other(
                    format!(
                        "{} seems to be installed by a package manager, cs2 won't be able to install/update it",
                        self.as_str()
                    ).as_str()
                ));
            }
        }
        Ok(())
    }

    pub fn install(&self, parallelism: bool) -> Result<(), Error> {
        let package = self.as_str();
        let temp_path = get_temp_path(package);
        let final_path = get_final_path(package);

        self.verify_install()?;

        if Path::new(&final_path).exists() {
            return Err(Error::other(
                "Already cloned and installed, use cs2 update instead",
            ));
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
                    return Err(Error::other("Couldn't chmod manual-install.sh"));
                }

                move_to_final_path(temp_path.as_str(), Path::new(&final_path))?;
            }
            Self::Banana => {
                clone_repo(BANANA_REPO, temp_path.as_str())?;
                move_to_final_path(temp_path.as_str(), Path::new(&final_path))?;
            }
            _ => return Err(Error::other("Cannot install package")),
        }

        self.build(parallelism)?;
        warn_path_var("/usr/local/bin");

        return Ok(());
    }

    pub fn update(&self, parallelism: bool, force: bool) -> Result<(), Error> {
        let package = self.as_str();
        let path = get_final_path(package);

        self.verify_install()?;

        if !Path::new(&path).exists() {
            return Err(Error::other(format!(
                "Impossible to find {}, have you installed it with cs2 install?",
                package
            )));
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
