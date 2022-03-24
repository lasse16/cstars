use reqwest::{blocking, cookie, Url};
use std::fs;
use std::sync::Arc;

use crate::errors::Error;

pub const ADVENT_OF_CODE_URL_BASE: &str = "https://adventofcode.com";
pub fn build_client() -> Result<blocking::Client, Error> {
    let cookie_jar = cookie::Jar::default();
    let url = ADVENT_OF_CODE_URL_BASE.parse::<Url>().expect(
        "Error parsing hardcoded AOC Url; This should never happen, open an issue immediately!",
    );
    let secret = fs::read_to_string("secret.txt").map_err(|err| Error::ConfigurationError {
        message: format!("Failed to get secret: {:?}", err.to_string()),
    })?;

    cookie_jar.add_cookie_str(&format!("session={}", &secret), &url);
    blocking::Client::builder()
        .cookie_provider(Arc::new(cookie_jar))
        .build()
        .map_err(|err| Error::ConfigurationError {
            message: err.to_string(),
        })
}
