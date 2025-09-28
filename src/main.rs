mod build_systems;
mod commands;
mod parse;
mod shared;
use clap::{Arg, ArgAction, Command, command};
use std::io::{BufRead, IsTerminal};

// TODO: simplify arguments in install & update

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
                )
                .arg(
                    Arg::new("parallelism")
                        .short('j')
                        .help("For banana, install with parallelism")
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
                )
                .arg(
                    Arg::new("parallelism")
                        .short('j')
                        .help("For banana, install with parallelism")
                        .num_args(0),
                ),
        )
        .subcommand(
            Command::new("run")
                .about("Run your command through the coding style checker")
                .arg(Arg::new("command").action(ArgAction::Append)),
        )
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
        Some(("run", sub_matches)) => {
            if !sub_matches.args_present() {
                println!("No command provided");
                std::process::exit(1);
            }

            let command_args = sub_matches
                .get_many::<String>("command")
                .unwrap_or_default()
                .collect::<Vec<_>>();

            if command_args.len() <= 0 {
                println!("No command provided");
                std::process::exit(1);
            }

            match commands::run::run(command_args) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
            };
        }
        _ => {
            if !std::io::stdin().is_terminal() {
                let mut full_input = Vec::new();
                for line in std::io::stdin().lock().lines() {
                    match line {
                        Ok(s) => full_input.push(s),
                        Err(_) => break,
                    }
                }

                let _ = parse::parse_output(full_input);
            } else {
                let lines = match build_systems::find() {
                    Ok(lines) => lines,
                    Err(e) => {
                        println!("{}", e);
                        std::process::exit(1);
                    }
                };

                let _ = parse::parse_output(lines);
            }
        }
    }
}
