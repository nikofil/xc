use std::fmt;
use crate::parser::Operator;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NumParseError(String),
    OperatorParseError(String),
    ExprParseError(Operator),
    ExprTermsError,
    UnmatchedParenthError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NumParseError(s) => write!(f, "Could not parse number {}", s),
            Error::OperatorParseError(s) => write!(f, "Unknown operator {}", s),
            Error::ExprParseError(o) => write!(f, "Could not parse {} expression", o),
            Error::ExprTermsError => write!(f, "Incorrect terms found in expression"),
            Error::UnmatchedParenthError => write!(f, "Unmatched parenthesis in expression"),
        }
    }
}
