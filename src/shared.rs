use std::io::Error;

pub const BANANA_ERROR_PREFIX: &str = "[Banana] ";
pub const DEFAULT_RUN_ENV: [(&str, &str); 1] = [("CC", "epiclang")];

pub fn split_output(output: Vec<u8>) -> Result<Vec<String>, Error> {
    // TODO: replace unwrap if possible
    let output_str = String::from_utf8(output).unwrap();

    Ok(output_str
        .split("\n")
        .map(|f| String::from(f))
        .collect::<Vec<_>>())
}

pub fn merge_outputs(stdout: Vec<u8>, stderr: Vec<u8>) -> Vec<u8> {
    let mut merged: Vec<u8> = Vec::new();

    stdout.iter().for_each(|c| merged.push(*c));
    stderr.iter().for_each(|c| merged.push(*c));

    merged
}
