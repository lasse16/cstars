use clap::{ArgEnum, Args, Error, Parser, Subcommand};
use lib_cstars::shared::OutputFormat;

pub fn parse_cli_arguments() -> Result<Cli, Error> {
    Cli::try_parse()
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

    /// Get a specific information for a date
    Get {
        #[clap(subcommand)]
        object: GetType,
    },
    /// Print the used configuration
    Config {},
}

#[derive(Clone, Debug, Subcommand)]
pub enum GetType {
    ///Get the input for a specific date
    Input {
        #[clap(flatten)]
        date: CliArgsDate,
    },

    ///Get the completed puzzle parts for a specific date
    StarCount {
        #[clap(flatten)]
        date: CliArgsDate,
    },

    ///Get the description for a specific date
    Description {
        #[clap(flatten)]
        date: CliArgsDate,
        output: Option<CliOutputType>,
    },
}

#[derive(Debug, Args, Clone)]
pub struct CliArgsDate {
    ///day
    day: u8,

    ///year
    year: u16,
}

#[derive(Debug, ArgEnum, Clone)]
pub enum CliOutputType {
    ///markdown
    Markdown,
    ///html
    Html,
}

impl std::str::FromStr for CliOutputType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "markdown" | "md" => CliOutputType::Markdown,
            "html" => CliOutputType::Html,
            _ => {
                return Err(Error::raw(
                    clap::ErrorKind::InvalidValue,
                    "Unrecognized output type",
                ))
            }
        })
    }
}

impl From<CliOutputType> for lib_cstars::shared::OutputFormat {
    fn from(date: CliOutputType) -> Self {
        match date {
            CliOutputType::Markdown => OutputFormat::Markdown,
            CliOutputType::Html => OutputFormat::Html,
        }
    }
}

impl From<CliArgsDate> for lib_cstars::shared::Date {
    fn from(date: CliArgsDate) -> Self {
        lib_cstars::shared::Date {
            day: date.day,
            year: date.year,
        }
    }
}
