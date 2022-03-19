use reqwest::{blocking, cookie, Url};
use std::fs;
use std::sync::Arc;

pub const ADVENT_OF_CODE_URL_BASE: &str = "https://adventofcode.com";
pub fn build_client() -> Result<blocking::Client, reqwest::Error> {
    let cookie_jar = cookie::Jar::default();
    let url = ADVENT_OF_CODE_URL_BASE.parse::<Url>().unwrap();
    let secret = fs::read_to_string("secret.txt").unwrap();
    cookie_jar.add_cookie_str(&format!("session={}", &secret), &url);
    Ok(blocking::Client::builder()
        .cookie_provider(Arc::new(cookie_jar))
        .build()?)
}