use reqwest::{blocking, cookie, Url};
use secrecy::{ExposeSecret, SecretString};
use std::process::Command;
use std::sync::Arc;

use crate::configuration;
use crate::errors::{Error, ErrorKind};
use crate::url;

pub fn build_client(config: &configuration::Configuration) -> Result<blocking::Client, Error> {
    let cookie_jar = cookie::Jar::default();
    let url = url::ADVENT_OF_CODE_URL_BASE.parse::<Url>().expect(
        "Error parsing hardcoded AOC Url; This should never happen, open an issue immediately!",
    );

    let secret = get_secret(&config.session_cookie_retrieval_command).map_err(|err| {
        Error::new(ErrorKind::Configuration {
            message: format!("Failed to get secret: {:?}", err.to_string()),
        })
    })?;
    log::trace!("Retrieved session secret");

    cookie_jar.add_cookie_str(&format!("session={}", &secret.expose_secret()), &url);
    log::trace!("Adding session secret cookie");
    blocking::Client::builder()
        .cookie_provider(Arc::new(cookie_jar))
        .build()
        .map_err(|err| {
            Error::new(ErrorKind::Configuration {
                message: err.to_string(),
            })
        })
}

fn get_secret(secret_retrieval_command: &str) -> Result<SecretString, std::io::Error> {
    // TODO It's linux only
    let command_output = Command::new("sh")
        .arg("-c")
        .arg(secret_retrieval_command)
        .output()?;
    if !command_output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            String::from_utf8(command_output.stderr).unwrap(),
        ));
    }
    let retrieved_secret = String::from_utf8(command_output.stdout).unwrap();
    Ok(SecretString::from(retrieved_secret))
}
