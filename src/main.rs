mod build_systems;
mod commands;
mod package;
mod parse;
mod shared;
use clap::{command, Arg, ArgAction, Command};
use std::io::{BufRead, IsTerminal};

// TODO: simplify arguments in install & update

fn main() {
    let jobs_amount = std::thread::available_parallelism()
        .unwrap()
        .get()
        .to_string();

    let matches = command!()
        .subcommand(
            Command::new("install")
                .about("Installs all the dependencies needed")
                .arg(
                    Arg::new("package")
                        .long("package")
                        .help("Only install a certain package")
                        .num_args(1),
                )
                .arg(
                    Arg::new("parallelism")
                        .short('j')
                        .help("For banana, install with parallelism")
                        .default_value("1")
                        .default_missing_value(&jobs_amount)
                        .num_args(0..=1),
                ),
        )
        .subcommand(
            Command::new("update")
                .about("Update cs2 and the dependencies")
                .arg(
                    Arg::new("package")
                        .long("package")
                        .help("Only install a certain package")
                        .num_args(1),
                )
                .arg(
                    Arg::new("parallelism")
                        .short('j')
                        .help("For banana, install with parallelism")
                        .default_value("1")
                        .default_missing_value(&jobs_amount)
                        .num_args(0..=1),
                )
                .arg(
                    Arg::new("force")
                        .short('f')
                        .long("force")
                        .help("Force update even if there is nothing new when fetching")
                        .num_args(0),
                ),
        )
        .subcommand(
            Command::new("run")
                .about("Run your command through the coding style checker")
                .arg(Arg::new("command").action(ArgAction::Append)),
        )
        .arg(
            Arg::new("no-ignore")
                .long("no-ignore")
                .help("Disable checking for files ignored by git")
                .num_args(0),
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

                let _ = parse::parse_output(full_input, true);
            } else {
                if !build_systems::verify_packages() {
                    println!(
                        "Some packages seem to not be installed, make sure you ran cs2 install before"
                    );
                    std::process::exit(1);
                }

                let lines = match build_systems::find() {
                    Ok(lines) => lines,
                    Err(e) => {
                        println!("{}", e);
                        std::process::exit(1);
                    }
                };

                match parse::parse_output(lines, matches.get_flag("no-ignore")) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}", e);
                        std::process::exit(1);
                    }
                };
            }
        }
    }
}
