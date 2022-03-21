#[derive(Debug)]
pub enum Error {
    ConfigurationError,
    ConnectionError,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConfigurationError => write!(f, "configuration error"),
            Error::ConnectionError => write!(f, "connection error"),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(_: reqwest::Error) -> Self {
        Error::ConnectionError
    }
}
