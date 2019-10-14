use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NumParseError(String),
    OperatorParseError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NumParseError(s) => write!(f, "Could not parse number {}", s),
            Error::OperatorParseError(s) => write!(f, "Unknown operator {}", s),
        }
    }
}
