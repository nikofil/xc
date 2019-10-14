use std::fmt;

pub type Result<'a, T> = std::result::Result<T, Error<'a>>;

#[derive(Debug)]
pub enum Error<'a> {
    NumParseError(&'a str),
    OperatorParseError(&'a str),
}

impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NumParseError(s) => write!(f, "Could not parse number {}", s),
            Error::OperatorParseError(s) => write!(f, "Unknown operator {}", s),
        }
    }
}
