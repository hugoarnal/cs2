mod build_systems;
mod ci;
mod commands;
mod package;
mod parse;
mod shared;
use ci::Ci;
use clap::{command, Arg, ArgAction, Command};
use std::{
    io::{BufRead, IsTerminal},
    str::FromStr,
};

fn main() {
    let jobs_amount = std::thread::available_parallelism()
        .unwrap()
        .get()
        .to_string();

    let package_arg = Arg::new("package")
        .long("package")
        .help("Only install a certain package")
        .num_args(1);

    let parallelism_arg = Arg::new("parallelism")
        .short('j')
        .long("jobs")
        .help("Compile, if possible, with parallelism")
        .default_value("1")
        .default_missing_value(&jobs_amount)
        .num_args(0..=1);

    let matches = command!()
        .subcommand(
            Command::new("install")
                .about("Installs all the dependencies needed")
                .arg(&package_arg)
                .arg(&parallelism_arg),
        )
        .subcommand(
            Command::new("update")
                .about("Update cs2 and the dependencies")
                .arg(&package_arg)
                .arg(&parallelism_arg)
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
        .arg(
            Arg::new("ci")
                .long("ci")
                .default_value("none")
                .default_missing_value("github")
                .help("CI mode, enables exit code & prints it for the specified platform")
                .num_args(0..=1),
        )
        .arg(&parallelism_arg)
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
            let ci_flag = matches.get_one::<String>("ci").unwrap();
            let ci: Option<Ci> = if ci_flag == "none" {
                None
            } else {
                Some(match Ci::from_str(&ci_flag) {
                    Ok(ci) => ci,
                    Err(e) => {
                        println!("{}", e);
                        std::process::exit(1);
                    }
                })
            };

            if !std::io::stdin().is_terminal() && ci.is_none() {
                let mut full_input = Vec::new();
                for line in std::io::stdin().lock().lines() {
                    match line {
                        Ok(s) => full_input.push(s),
                        Err(_) => break,
                    }
                }

                let _ = parse::parse_output(full_input, true, None);
            } else {
                if !build_systems::verify_packages() {
                    println!(
                        "Some packages seem to not be installed, make sure you ran cs2 install before"
                    );
                    std::process::exit(1);
                }

                let lines = match build_systems::find(
                    matches
                        .get_one::<String>("parallelism")
                        .unwrap()
                        .to_string(),
                ) {
                    Ok(lines) => lines,
                    Err(e) => {
                        println!("{}", e);
                        std::process::exit(1);
                    }
                };

                match parse::parse_output(lines, matches.get_flag("no-ignore"), ci) {
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
