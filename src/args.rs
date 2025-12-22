use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum ArgSubcommand {
    /// Installs all the dependencies needed
    Install {
        /// Only install a certain package
        #[arg(long)]
        package: Option<String>,

        /// Compile, if possible, with parallelism
        #[arg(short, long, default_missing_value = "", num_args = 0..=1)]
        jobs: Option<String>,
    },
    /// Update cs2 and the dependencies
    Update {
        /// Only update a certain package
        #[arg(long)]
        package: Option<String>,

        /// Compile, if possible, with parallelism
        #[arg(short, long, default_missing_value = "", num_args = 0..=1)]
        jobs: Option<String>,

        /// Force update even if there is nothing new when fetching
        #[arg(short, long)]
        force: bool,
    },
    /// Run your command through the cs2 helper
    Run {
        #[arg(action = clap::ArgAction::Append)]
        command: Vec<String>,
    },
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub(crate) command: Option<ArgSubcommand>,

    /// Compile, if possible, with parallelism
    #[arg(short, long, default_missing_value = "", num_args = 0..=1)]
    pub(crate) jobs: Option<String>,

    /// Prints the errors in a correct way for the specified platform
    #[arg(long)]
    pub(crate) ci: Option<String>,

    /// Disable checking for files ignored by git
    #[arg(long)]
    pub(crate) no_ignore: bool,

    /// Ignore rules, must be comma separated if there are multiple
    /// Ignored rules won't show up in "ignored errors" as it's ignored directly by Banana and not cs2
    #[arg(long)]
    pub(crate) ignore_rules: Option<String>,

    /// Ignore paths, must be comma separated if there are multiple
    /// Ignored paths won't show up in "ignored errors" as it's ignored directly by Banana and not cs2
    #[arg(long)]
    pub(crate) ignore_paths: Option<String>,
}

pub fn get_jobs_number(jobs: &Option<String>) -> String {
    if let Some(jobs) = jobs {
        if jobs.is_empty() {
            std::thread::available_parallelism()
                .unwrap()
                .get()
                .to_string()
        } else {
            jobs.to_string()
        }
    } else {
        "1".to_string()
    }
}
