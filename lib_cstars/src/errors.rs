use std::error;

pub trait Error: std::error::Error {}

#[derive(Debug)]
pub struct ConfigurationError {}

impl Error for ConfigurationError {}
impl error::Error for ConfigurationError {}
impl std::fmt::Display for ConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Configuration error")
    }
}

#[derive(Debug)]
pub struct ConnectionError {}

impl Error for ConnectionError {}
impl error::Error for ConnectionError {}
impl std::fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Connection error")
    }
}

impl From<reqwest::Error> for ConnectionError {
    fn from(_: reqwest::Error) -> Self {
        ConnectionError {}
    }
}
