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
