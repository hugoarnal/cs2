use std::io::Error;
use std::process::Command;

pub fn get_temp_path(package: &str) -> String {
    format!("/tmp/cs2-{}", package)
}

pub fn get_final_path(package: &str) -> String {
    format!("/usr/local/share/cs2/{}", package)
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

pub fn build_banana(final_path: &str) -> Result<(), Error> {
    let build_command = format!("cd {} && ./scripts/make_plugin.sh", final_path);

    if !Command::new("sh")
        .args(["-c", build_command.as_str()])
        .status()?
        .success()
    {
        return Err(Error::other("Impossible to build banana"));
    }

    if !Command::new("sudo")
        .args([
            "mv",
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

pub fn build_package(package: &str) -> Result<(), Error> {
    let final_path = get_final_path(package);

    if package == "epiclang" {
        build_epiclang(&final_path)?;
    } else if package == "banana" {
        build_banana(&final_path)?;
    }

    Ok(())
}
