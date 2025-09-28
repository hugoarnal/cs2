use std::fs;
use std::io::Error;
use std::process::Command;

fn from_output(output: Vec<u8>) -> Result<Vec<String>, Error> {
    // TODO: replace unwrap if possible
    let stdout_string = String::from_utf8(output).unwrap();
    let str_vec: Vec<&str> = stdout_string.split("\n").collect();
    // TODO: replace all this please
    let mut lines = Vec::new();

    for line in str_vec {
        println!("line: {}", line);
        lines.push(String::from(line));
    }

    Ok(lines)
}

// TODO: use enums

pub fn find() -> Result<Vec<String>, Error> {
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        let file_name = path.unwrap().file_name().to_ascii_lowercase();
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
