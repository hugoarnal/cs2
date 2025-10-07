use std::env;
use std::io::Error;
use std::process::Command;

pub fn get_temp_path(package: &str) -> String {
    format!("/tmp/cs2-{}", package)
}

pub fn get_final_path(package: &str) -> String {
    format!("/usr/local/share/cs2/{}", package)
}

fn patch_file(patch_name: &str, file: &str) -> Result<(), Error> {
    let command = Command::new("patch")
        .args([
            "-p0",
            "-s",
            "-f",
            file,
            format!("{}/src/patches/{}", get_final_path("cs2"), patch_name).as_str(),
        ])
        .status()?;

    if !command.success() {
        println!("{} already applied to {}", patch_name, file);
        return Ok(());
    }

    println!("Applied {} to {}", patch_name, file);

    Ok(())
}

pub fn build_epiclang(final_path: &str) -> Result<(), Error> {
    let build_command = format!("cd {} && sudo ./manual-install.sh", final_path);

    if !Command::new("sh")
        .args(["-c", build_command.as_str()])
        .status()?
        .success()
    {
        return Err(Error::other("Impossible to install epiclang"));
    }

    Ok(())
}

pub fn build_banana(final_path: &str, parallelism: bool) -> Result<(), Error> {
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

    let banana_check_repo_file = format!("{}/src/banana-check-repo", final_path);

    patch_file(
        "banana-check-repo-remove-leading-dot.patch",
        banana_check_repo_file.as_str(),
    )?;

    if !Command::new("sudo")
        .args([
            "cp",
            banana_check_repo_file.as_str(),
            "/usr/local/bin/banana-check-repo",
        ])
        .status()?
        .success()
    {
        return Err(Error::other("Impossible to move banana-check-repo"));
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

    Ok(())
}

fn build_cs2(final_path: &str) -> Result<(), Error> {
    let build_command = format!("cd {} && ./compile.sh", final_path);

    if !Command::new("sh")
        .args(["-c", build_command.as_str()])
        .status()?
        .success()
    {
        return Err(Error::other("Impossible to build cs2"));
    }

    Ok(())
}

pub fn build_package(package: &str, parallelism: bool) -> Result<(), Error> {
    let final_path = get_final_path(package);

    if package == "epiclang" {
        build_epiclang(&final_path)?;
    } else if package == "banana" {
        build_banana(&final_path, parallelism)?;
    } else if package == "cs2" {
        build_cs2(&final_path)?;
    }

    Ok(())
}

pub fn warn_path_var(directory: &str) {
    if !env::var("PATH").unwrap().contains(directory) {
        println!(
            "You need to add {} to your PATH environment variable.",
            directory
        );
    }
}
