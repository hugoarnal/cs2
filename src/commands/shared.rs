use anyhow::Result;
use std::env;

pub fn get_temp_path(package: &str) -> String {
    format!("/tmp/cs2-{}", package)
}

pub fn get_final_path(package: &str) -> String {
    format!("/usr/local/share/cs2/{}", package)
}

pub fn warn_path_var(directory: &str) -> Result<()> {
    if !env::var("PATH")?.contains(directory) {
        println!(
            "You need to add {} to your PATH environment variable.",
            directory
        );
    }
    Ok(())
}
