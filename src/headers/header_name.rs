use std::fmt::{self, Debug, Display};
use std::str::FromStr;

use crate::headers::ParseError;

/// A header name.
#[derive(Eq, PartialEq, Hash)]
pub struct HeaderName {
    /// The inner representation of the string.
    pub(crate) string: String,
    /// A const-friendly string. Useful because `String::from` cannot be used in const contexts.
    pub(crate) static_str: Option<&'static str>,
}

impl HeaderName {
    /// Create a new `HeaderName`.
    pub fn from_ascii(mut bytes: Vec<u8>) -> Result<Self, ParseError> {
        if !bytes.is_ascii() {
            return Err(ParseError::new());
        }
        bytes.make_ascii_lowercase();
        let string = String::from_utf8(bytes).map_err(|_| ParseError::new())?;
        Ok(Self {
            string: string,
            static_str: None,
        })
    }

    /// Converts a vector of bytes to a `HeaderName` without checking that the string contains
    /// valid ASCII.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check that the bytes passed to it are valid
    /// ASCII. If this constraint is violated, it may cause memory
    /// unsafety issues with future users of the HeaderName, as the rest of the library assumes
    /// that Strings are valid ASCII.
    pub unsafe fn from_ascii_unchecked(mut bytes: Vec<u8>) -> Self {
        bytes.make_ascii_lowercase();
        let string = String::from_utf8_unchecked(bytes);
        Self {
            string,
            static_str: None,
        }
    }
}

impl Display for HeaderName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(string) = self.static_str {
            Display::fmt(string, f)
        } else {
            Display::fmt(&self.string, f)
        }
    }
}

impl Debug for HeaderName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(string) = self.static_str {
            Debug::fmt(string, f)
        } else {
            Debug::fmt(&self.string, f)
        }
    }
}

impl FromStr for HeaderName {
    type Err = ParseError;

    /// Create a new `HeaderName`.
    ///
    /// This checks it's valid ASCII, and lowercases it.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(ParseError::new());
        }
        Ok(Self {
            string: s.to_ascii_lowercase(),
            static_str: None,
        })
    }
}
