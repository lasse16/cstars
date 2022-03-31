use figment::providers::{Format, Toml};
use serde::Deserialize;
use std::path::Path;

use crate::errors::Error;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    session_cookie_retrieval_command: String,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            session_cookie_retrieval_command: String::from("cat secret.txt"),
        }
    }
}

pub fn parse_configuration(toml_file_location: &Path) -> Result<Configuration, Error> {
    let default_configuration = Configuration::default();
    Ok(default_configuration)
}
