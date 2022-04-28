use crate::shared;
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    Configuration { message: String },
    Connection { message: String },
    Command { kind: CommandErrorKind },
}

#[derive(Debug)]
pub enum CommandErrorKind {
    MissingDate(shared::Date),
    UnknownPart(u8),
}

impl std::error::Error for Error {}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn new(kind: ErrorKind) -> Self {
        Error { kind }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::Configuration { message } => write!(f, "configuration error: {}", message),
            ErrorKind::Connection { message } => write!(f, "connection error: {}", message),
            ErrorKind::Command { kind } => write!(f, "command error: {:?}", kind),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if err.is_connect() {
            return Error::new(ErrorKind::Connection {
                message: err.to_string(),
            });
        }
        Error::new(ErrorKind::Configuration {
            message: err.to_string(),
        })
    }
}
