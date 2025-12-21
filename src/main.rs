mod args;
mod build_systems;
mod ci;
mod commands;
mod package;
mod parse;
mod shared;

use ci::Ci;
use clap::Parser;
use std::{
    io::{BufRead, IsTerminal},
    str::FromStr,
};

use crate::args::{get_jobs_number, ArgSubcommand, Args};

fn main() {
    let args = Args::parse();

    match &args.command {
        Some(ArgSubcommand::Install { package, jobs }) => {
            match commands::install::handler(package, &get_jobs_number(jobs)) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
            };
        }
        Some(ArgSubcommand::Update {
            package,
            jobs,
            force,
        }) => {
            match commands::update::handler(package, &get_jobs_number(jobs), *force) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
            };
        }
        Some(ArgSubcommand::Run { command }) => {
            if command.is_empty() {
                println!("No command provided");
                std::process::exit(1);
            }

            match commands::run::run(command) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
            };
        }
        &None => {
            let ci: Option<Ci> = if let Some(ref ci) = args.ci {
                match Ci::from_str(ci) {
                    Ok(ci) => Some(ci),
                    Err(_) => {
                        println!(
                            "Incorrect CI platform, continuing with no CI platform specified."
                        );
                        None
                    }
                }
            } else {
                None
            };

            // Piped input (<test command> | cs2)
            //
            // The reason for --ci cancelling any piped input
            // is due to CI runners doing the following to run commands:
            // `echo "cs2" | bash`
            //
            // This causes is_terminal to be false which triggers the "piped input mode"
            if !std::io::stdin().is_terminal() && ci.is_none() {
                let mut full_input = Vec::new();
                for line in std::io::stdin().lock().lines() {
                    match line {
                        Ok(s) => full_input.push(s),
                        Err(_) => break,
                    }
                }

                let _ = parse::parse_output(full_input, true, None);

            // Build system checking and running
            } else {
                if !build_systems::verify_packages() {
                    println!(
                        "Some packages seem to not be installed, make sure you ran cs2 install before"
                    );
                    std::process::exit(1);
                }

                let lines = match build_systems::find(&args) {
                    Ok(lines) => lines,
                    Err(e) => {
                        println!("{}", e);
                        std::process::exit(1);
                    }
                };

                match parse::parse_output(lines, args.no_ignore, ci) {
                    Ok(exit) => {
                        if exit {
                            std::process::exit(1);
                        }
                    }
                    Err(e) => {
                        println!("{}", e);
                        std::process::exit(1);
                    }
                };
            }
        }
    }
}
