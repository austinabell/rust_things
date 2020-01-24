use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    FailedToStart,
    Custom(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::FailedToStart => write!(f, "Failed to start service"),
            Error::Custom(msg) => write!(f, "Custom Service error: {}", msg),
        }
    }
}
