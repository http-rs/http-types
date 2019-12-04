use std::str::FromStr;

use crate::headers::ParseError;

/// A header name.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct HeaderName {
    pub(crate) string: String,
}

impl HeaderName {
    /// Create a new `HeaderName`.
    pub fn from_ascii(bytes: &[u8]) -> Result<Self, ParseError> {
        if !bytes.is_ascii() {
            return Err(ParseError::new());
        }
        let string =
            String::from_utf8(bytes.to_ascii_lowercase()).map_err(|_| ParseError::new())?;
        Ok(Self { string: string })
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
    pub unsafe fn from_ascii_unchecked(bytes: Vec<u8>) -> Self {
        let string = String::from_utf8_unchecked(bytes);
        Self { string }
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
        })
    }
}
