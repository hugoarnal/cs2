use std::process::Command;

use crate::packages::{
    binary::{download_html_ppa, untar_ppa},
    Package, PackageType,
};

use anyhow::{anyhow, Result};

const BANANA_PPA_LINK: &str =
    "https://ppa.launchpadcontent.net/epitech/ppa/ubuntu/pool/main/b/banana-coding-style-checker/";
const BANANA_FINAL_TAR_FILE: &str = "/tmp/banana.tar.xz";
const TEMP_BANANA_DIR: &str = "/tmp/banana-binary";

pub struct BananaBinary {}

impl BananaBinary {
    pub fn new() -> Self {
        BananaBinary {}
    }
}

impl Package for BananaBinary {
    fn as_str(&self) -> &'static str {
        "banana-bin"
    }

    fn get_type(&self) -> PackageType {
        PackageType::Binary
    }

    fn set_parallelism(&mut self, _: &str) {}

    fn download(&self) -> Result<()> {
        download_html_ppa(
            BANANA_PPA_LINK,
            "/tmp/banana-ppa-result.html",
            BANANA_FINAL_TAR_FILE,
        )
    }

    fn build(&self) -> Result<()> {
        untar_ppa(TEMP_BANANA_DIR, BANANA_FINAL_TAR_FILE)
    }

    fn install(&self) -> Result<()> {
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
        Ok(())
    }
}
