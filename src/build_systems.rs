use std::fs;
use std::io::Error;
use std::process::Command;

fn from_output(output: Vec<u8>) -> Result<Vec<String>, Error> {
    // TODO: replace unwrap if possible
    let output_str = String::from_utf8(output).unwrap();

    Ok(output_str
        .split("\n")
        .map(|f| String::from(f))
        .collect::<Vec<_>>())
}

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
    // let lines = from_output(version_output.stdout)?;

    // Ok(lines)

    Err(Error::other("yeah"))
}
