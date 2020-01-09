use std::borrow::Cow;
use std::fmt::{self, Debug, Display};
use std::str::FromStr;

use crate::headers::ParseError;

/// A header name.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HeaderName(Cow<'static, str>);

impl HeaderName {
    /// Create a new `HeaderName`.
    pub fn from_ascii(mut bytes: Vec<u8>) -> Result<Self, ParseError> {
        if !bytes.is_ascii() {
            return Err(ParseError::new());
        }
        bytes.make_ascii_lowercase();
        let string = String::from_utf8(bytes).map_err(|_| ParseError::new())?;
        Ok(HeaderName(Cow::Owned(string)))
    }

    /// Returns the header name as a `&str`.
    pub fn as_str(&self) -> &'_ str {
        &self.0
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
        HeaderName(Cow::Owned(string))
    }

    /// Converts a string assumed to lowercase into a `HeaderName`
    pub(crate) const fn from_lowercase_str(str: &'static str) -> Self {
        HeaderName(Cow::Borrowed(str))
    }
}

impl Display for HeaderName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
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
        Ok(HeaderName(Cow::Owned(s.to_ascii_lowercase())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_name_static_non_static() {
        let static_header = HeaderName::from_lowercase_str("hello");
        let non_static_header = HeaderName::from_str("hello").unwrap();

        assert_eq!(&static_header, &non_static_header);
        assert_eq!(&static_header, &static_header);
        assert_eq!(&non_static_header, &non_static_header);

        assert_eq!(static_header, non_static_header);
        assert_eq!(static_header, static_header);
        assert_eq!(non_static_header, non_static_header);
    }
}
