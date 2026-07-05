use std::{path::Path, process::Command};

use crate::{
    commands::shared::{get_final_path, get_temp_path},
    packages::{source::clone_repo, Package, PackageType},
    shared::move_to_final_path,
};

use anyhow::{anyhow, Result};

const EPICLANG_REPO: &str = "git@github.com:Epitech/epiclang.git";

pub struct EpiclangSource {
    parallelism: String,
    temp_path: String,
    final_path: String,
}

impl EpiclangSource {
    pub fn new() -> Self {
        EpiclangSource {
            parallelism: String::new(),
            temp_path: get_temp_path("epiclang-src"),
            final_path: get_final_path("epiclang-src"),
        }
    }
}

impl Package for EpiclangSource {
    fn as_str(&self) -> &'static str {
        "epiclang-src"
    }

    fn get_type(&self) -> PackageType {
        PackageType::Source
    }

    fn set_parallelism(&mut self, parallelism: &str) {
        self.parallelism = String::from(parallelism);
    }

    fn download(&self) -> Result<()> {
        clone_repo(EPICLANG_REPO, self.temp_path.as_str())?;
        move_to_final_path(self.temp_path.as_str(), Path::new(&self.final_path))?;
        Ok(())
    }

    fn build(&self) -> Result<()> {
        // No need to do a building step, it's a Python script
        Ok(())
    }

    fn install(&self) -> Result<()> {
        let build_command = format!(
            "cd {} && sudo sh ./manual-install.sh",
            get_final_path(self.as_str())
        );

        if !Command::new("sh")
            .args(["-c", build_command.as_str()])
            .status()?
            .success()
        {
            return Err(anyhow!("Encountered an error installing epiclang"));
        }
        Ok(())
    }
}
