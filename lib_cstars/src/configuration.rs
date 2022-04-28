use directories;
use figment::providers::{Format, Toml};
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::errors::{Error, ErrorKind};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Configuration {
    pub session_cookie_retrieval_command: String,
    pub cache_dir: std::path::PathBuf,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            session_cookie_retrieval_command: String::from("cat secret.txt"),
            cache_dir: std::path::PathBuf::from(
                directories::ProjectDirs::from("", "cstars", "cstars")
                    .unwrap()
                    .cache_dir(),
            ),
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
        .map_err(|err| {
            Error::new(ErrorKind::Configuration {
                message: err.to_string(),
            })
        })?;

    Ok(config)
}
