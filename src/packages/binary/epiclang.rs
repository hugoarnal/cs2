use std::process::Command;

use crate::packages::{
    binary::{download_html_ppa, untar_ppa},
    Package, PackageType,
};

use anyhow::{anyhow, Result};

const EPICLANG_PPA_LINK: &str =
    "https://ppa.launchpadcontent.net/epitech/ppa/ubuntu/pool/main/e/epiclang/";
const EPICLANG_FINAL_TAR_FILE: &str = "/tmp/epiclang.tar.xz";
const TEMP_EPICLANG_DIR: &str = "/tmp/epiclang-binary";

pub struct EpiclangBinary {}

impl EpiclangBinary {
    pub fn new() -> Self {
        EpiclangBinary {}
    }
}

impl Package for EpiclangBinary {
    fn as_str(&self) -> &'static str {
        "epiclang-bin"
    }

    fn get_type(&self) -> PackageType {
        PackageType::Binary
    }

    fn set_parallelism(&mut self, _: &str) {}

    fn download(&self) -> Result<()> {
        download_html_ppa(
            EPICLANG_PPA_LINK,
            "/tmp/epiclang-ppa-result.html",
            EPICLANG_FINAL_TAR_FILE,
        )
    }

    fn build(&self) -> Result<()> {
        untar_ppa(TEMP_EPICLANG_DIR, EPICLANG_FINAL_TAR_FILE)
    }

    fn install(&self) -> Result<()> {
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

        Ok(())
    }
}
