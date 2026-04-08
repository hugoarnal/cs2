use anyhow::Result;
use std::process::{Child, Command, Output, Stdio};

// The reason why patches are directly in the code is because I want
// anyone to just download cs2 and use patches without the need for
// external files.
//
// Patches content go after this line (with description if possible)

fn echo_patch(next_command: &mut Child, patch: &str) -> Result<Output> {
    Ok(Command::new("echo")
        .args([patch])
        .stdout(next_command.stdin.take().unwrap())
        .spawn()?
        .wait_with_output()?)
}

pub fn is_patched(file: &str, patch: &str) -> Result<bool> {
    let mut dryrun_patch = Command::new("patch")
        .args(["-R", "-p0", "-s", "-f", "--dry-run", file])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()?;
    let _ = echo_patch(&mut dryrun_patch, patch);

    Ok(!dryrun_patch.wait_with_output()?.status.success())
}

#[allow(dead_code)]
pub fn apply_patch(file: &str, patch: &str) -> Result<bool> {
    if is_patched(file, patch)? {
        let mut actual_patch = Command::new("sudo")
            .args(["patch", "-p0", "-s", "-f", file])
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .spawn()?;
        let _ = echo_patch(&mut actual_patch, patch);
        return Ok(actual_patch.wait()?.success());
    }
    Ok(false)
}
