use std::fmt;

// Can define different types of error in enums
#[derive(Debug, PartialEq)]
pub enum Error {
    BaseError,
    ParameterError(String),
    TwoParameterError(String, u8),
    StructError { name: String, number: u8 },
    NestedError(OtherError),
    Other,
}

// Implementing to_string for Error
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::BaseError => write!(f, "Base Error"),
            // * For non copyable data, make sure that a reference of the object is used
            // * Alternatively, you could match on the reference to self instead
            Error::ParameterError(ref s) => write!(f, "String parameter error: {}", s),
            Error::TwoParameterError(ref s, n) => write!(f, "String: {}, number: {}", s, n),
            Error::StructError {
                name: ref n,
                number: num,
            } => write!(f, "name: {}, num: {}", n, num),
            Error::NestedError(ref err) => write!(f, "Nested error, err inside is: {}", err),
            Error::Other => write!(f, "Unknown error"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum OtherError {
    SimpleError,
}

impl fmt::Display for OtherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            OtherError::SimpleError => write!(f, "Other simple error"),
        }
    }
}
