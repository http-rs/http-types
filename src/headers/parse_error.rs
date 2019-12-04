use std::error::Error;
use std::fmt::{self, Display};

/// An error returned when failing to convert into an HTTP header.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ParseError {
    _private: (),
}

impl ParseError {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "Error parsing a string into an HTTP value".fmt(f)
    }
}
