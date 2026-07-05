mod banana;
mod binary;
mod epiclang;
pub mod source;

use std::path::Path;

use anyhow::{anyhow, Result};

#[derive(PartialEq)]
pub enum PackageType {
    Binary,
    Source,
}

pub trait Package {
    fn as_str(&self) -> &'static str;

    fn get_type(&self) -> PackageType;
    fn set_parallelism(&mut self, parallelism: &str);

    fn download(&self) -> Result<()>;
    fn build(&self) -> Result<()>;
    fn install(&self) -> Result<()>;
}

/// Consider the binary packages as default packages
pub fn get_default_packages() -> [Box<dyn Package>; 2] {
    [
        Box::new(binary::epiclang::EpiclangBinary::new()),
        Box::new(binary::banana::BananaBinary::new()),
    ]
}

pub fn get_all_packages() -> [Box<dyn Package>; 4] {
    [
        Box::new(binary::epiclang::EpiclangBinary::new()),
        Box::new(binary::banana::BananaBinary::new()),
        Box::new(source::epiclang::EpiclangSource::new()),
        Box::new(source::banana::BananaSource::new()),
    ]
}

pub fn from_str(str: &String) -> Result<Box<dyn Package>> {
    let packages = get_all_packages();

    for package in packages {
        if package.as_str() == str {
            return Ok(package);
        }
    }
    Err(anyhow!("Impossible to find package"))
}

pub fn verify_installation() -> bool {
    let packages_paths: Vec<Vec<&'static str>> = vec![
        banana::get_binary_locations(),
        banana::get_plugin_locations(),
        epiclang::get_binary_locations(),
    ];

    for package in packages_paths {
        let mut found = false;

        for path in package {
            if Path::new(path).exists() {
                found = true;
            }
        }

        if !found {
            return false;
        }
    }
    true
}
