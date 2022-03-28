#[derive(Debug)]
pub enum Error {
    ConfigurationError { message: String },
    ConnectionError { message: String },
    CommandError { message: String },
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConfigurationError { message } => write!(f, "configuration error: {}", message),
            Error::ConnectionError { message } => write!(f, "connection error: {}", message),
            Error::CommandError { message } => write!(f, "command error: {}", message),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if err.is_connect() {
            return Error::ConnectionError {
                message: err.to_string(),
            };
        }
        Error::ConfigurationError {
            message: err.to_string(),
        }
    }
}
