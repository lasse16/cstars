use figment::providers::{Format, Toml};
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::errors::Error;

#[derive(Debug, Deserialize, Serialize)]
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

impl figment::Provider for Configuration {
    fn metadata(&self) -> figment::Metadata {
        figment::Metadata::named("lib_cstars configuration")
    }

    fn data(
        &self,
    ) -> Result<figment::value::Map<figment::Profile, figment::value::Dict>, figment::Error> {
        figment::providers::Serialized::defaults(Configuration::default()).data()
    }
}

pub fn parse_configuration(toml_file_location: &Path) -> Result<Configuration, Error> {
    let default_configuration = Configuration::default();
    let config: Configuration = figment::Figment::from(default_configuration)
        .merge(Toml::file(toml_file_location))
        .extract()
        .map_err(|err| Error::ConfigurationError {
            message: err.to_string(),
        })?;

    Ok(config)
}
