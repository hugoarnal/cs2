use std::{path::Path, process::Command};

use crate::{
    commands::shared::{get_final_path, get_temp_path},
    packages::{source::clone_repo, Package, PackageType},
    shared::move_to_final_path,
};

use anyhow::{anyhow, Result};

const BANANA_REPO: &str = "git@github.com:Epitech/banana-coding-style-checker.git";

pub struct BananaSource {
    parallelism: String,
    temp_path: String,
    final_path: String,
}

impl BananaSource {
    pub fn new() -> Self {
        BananaSource {
            parallelism: String::new(),
            temp_path: get_temp_path("banana-src"),
            final_path: get_final_path("banana-src"),
        }
    }
}

impl Package for BananaSource {
    fn as_str(&self) -> &'static str {
        "banana-src"
    }

    fn get_type(&self) -> PackageType {
        PackageType::Source
    }

    fn set_parallelism(&mut self, parallelism: &str) {
        self.parallelism = String::from(parallelism);
    }

    fn download(&self) -> Result<()> {
        clone_repo(BANANA_REPO, self.temp_path.as_str())?;
        move_to_final_path(self.temp_path.as_str(), Path::new(&self.final_path))?;
        Ok(())
    }

    fn build(&self) -> Result<()> {
        let build_command = format!("cd {} && ./scripts/make_plugin.sh", self.final_path);

        let mut full_command = Command::new("sh");
        full_command.args(["-c", build_command.as_str()]);

        full_command.env("CMAKE_BUILD_PARALLEL_LEVEL", self.parallelism.as_str());

        if !full_command.status()?.success() {
            return Err(anyhow!("Encountered an error building banana plugin",));
        }
        Ok(())
    }

    fn install(&self) -> Result<()> {
        if !Command::new("sudo")
            .args([
                "install",
                "-Dm755",
                format!("{}/epiclang-plugin-banana.so", self.final_path).as_str(),
                "/usr/local/lib/epiclang/plugins/epiclang-plugin-banana.so",
            ])
            .status()?
            .success()
        {
            return Err(anyhow!(
                "Impossible to move epiclang-plugin-banana.so to it's destination",
            ));
        }

        if !Path::new(&self.final_path).exists() {
            return Err(anyhow!(
                "Impossible to find banana repo, are you sure it is installed?",
            ));
        }

        let file_name = format!("{}/src/banana-check-repo", self.final_path);

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
            return Err(anyhow!(
                "Impossible to move banana-check-repo to it's destination",
            ));
        }
        Ok(())
    }
}
