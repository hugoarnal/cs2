mod commands;
use clap::{Command, command};
use std::io::IsTerminal;

fn main() {
    let matches = command!()
        .subcommand(Command::new("install").about("Installs all the dependencies needed"))
        .subcommand(Command::new("update").about("Update cs2 and the dependencies"))
        .subcommand(Command::new("run").about("Run your command through the coding style checker"))
        .get_matches();

    match matches.subcommand() {
        Some(("install", _)) => {
            match commands::install::all() {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
            };
        }
        Some(("update", _)) => {
            match commands::update::all() {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
            };
        }
        _ => {
            if !std::io::stdin().is_terminal() {
                println!("This is a pipe");
            } else {
                println!("Find a build system");
            }
        }
    }
}
