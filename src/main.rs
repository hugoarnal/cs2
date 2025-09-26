mod commands;
use clap::{Command, command};

fn main() {
    let matches = command!()
        .subcommand(Command::new("install").about("Installs all the dependencies needed"))
        .get_matches();

    match matches.subcommand() {
        Some(("install", _)) => {
            if !commands::install::all() {
                std::process::exit(1);
            };
        }
        _ => {
            println!("Hello, world!");
        }
    }
}
