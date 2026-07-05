use std::{fs, process::Command};

use crate::shared::download_file;
use anyhow::anyhow;
use regex::Regex;

pub mod banana;
pub mod epiclang;

const TAR_XZ_PPA_REGEX: &str = r"<a[^>]+>(.+?\.tar\.xz)</a>";

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

// TODO: we could move this to https://crates.io/crates/tar
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
