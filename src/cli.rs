use clap::{ArgEnum, Args, Parser, Subcommand};

pub fn parse_cli_arguments() -> Cli {
    Cli::parse()
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[clap(short, long, parse(from_occurrences))]
    verbose: usize,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Submit an answer for a date
    Submit {
        solution: String,

        #[clap(flatten)]
        date: CliArgsDate,
    },

    /// Get a specific object for a date
    Get {
        #[clap(arg_enum)]
        object: GetType,

        #[clap(flatten)]
        date: CliArgsDate,
    },
    /// Interact with the config file
    Config {},
}

#[derive(Clone, Debug, ArgEnum)]
pub enum GetType {
    Input,
    Description,
}

#[derive(Debug, Args)]
pub struct CliArgsDate {
    ///day
    day: u8,

    ///year
    year: u16,
}

impl From<CliArgsDate> for lib_cstars::shared::Date {
    fn from(date: CliArgsDate) -> Self {
        lib_cstars::shared::Date {
            day: date.day,
            year: date.year,
        }
    }
}
