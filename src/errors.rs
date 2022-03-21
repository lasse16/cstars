use lib_cstars::errors::Error as lib_error;
use std::error::Error;

#[derive(Debug)]
pub enum CliError {
    CliArgsError,
    LibraryError(lib_error),
}

impl Error for CliError {}
impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::CliArgsError => write!(f, "CliArgsError"),
            CliError::LibraryError(err) => write!(f, "LibraryError {}", err),
        }
    }
}

impl From<lib_error> for CliError {
    fn from(err: lib_error) -> Self {
        CliError::LibraryError(err)
    }
}

impl From<clap::Error> for CliError {
    fn from(_: clap::Error) -> Self {
        CliError::CliArgsError
    }
}
