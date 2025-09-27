mod commands;
use clap::{Arg, Command, command};
use std::io::IsTerminal;

fn main() {
    let matches = command!()
        .subcommand(
            Command::new("install")
                .about("Installs all the dependencies needed")
                .arg(
                    Arg::new("epiclang")
                        .long("epiclang")
                        .help("Only install epiclang")
                        .num_args(0),
                )
                .arg(
                    Arg::new("banana")
                        .long("banana")
                        .help("Only install banana")
                        .num_args(0),
                ),
        )
        .subcommand(
            Command::new("update")
                .about("Update cs2 and the dependencies")
                .arg(
                    Arg::new("cs2")
                        .long("cs2")
                        .help("Only update cs2")
                        .num_args(0),
                )
                .arg(
                    Arg::new("epiclang")
                        .long("epiclang")
                        .help("Only update epiclang")
                        .num_args(0),
                )
                .arg(
                    Arg::new("banana")
                        .long("banana")
                        .help("Only update banana")
                        .num_args(0),
                ),
        )
        .subcommand(Command::new("run").about("Run your command through the coding style checker"))
        .get_matches();

    match matches.subcommand() {
        Some(("install", args)) => {
            match commands::install::handler(&args) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
            };
        }
        Some(("update", args)) => {
            match commands::update::handler(&args) {
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
