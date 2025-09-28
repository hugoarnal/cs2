use std::fs;
use std::io::Error;
use std::process::Command;

// TODO: use enums

pub fn find() -> Result<Vec<String>, Error> {
    let paths = fs::read_dir("./")?;

    for path in paths {
        let file_name = path?.file_name().to_ascii_lowercase();
        if file_name == "makefile" || file_name == "gnumakefile" {
            break;
        }
        if file_name == "cmakelists.txt" {
            break;
        }
    }

    // let version_output = Command::new("make").output()?;
    // let lines = shared::parse_output(version_output.stdout)?;

    // Ok(lines)

    Err(Error::other("yeah"))
}
