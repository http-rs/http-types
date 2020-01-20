use std::fmt::{self, Display};
use std::str::FromStr;

use crate::{Error, ErrorKind};
use crate::{Cookie, Mime};

/// A header value.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct HeaderValue {
    inner: String,
}

impl HeaderValue {
    /// Create a new `HeaderValue` from ASCII bytes.
    ///
    /// # Error
    ///
    /// This function will error if the string is not a valid ASCII.
    pub fn from_ascii(bytes: &[u8]) -> Result<Self, Error> {
        if !bytes.is_ascii() {
            return Err(Error::from(ErrorKind::InvalidData));
        }

        // This is permitted because ASCII is valid UTF-8, and we just checked that.
        let string = unsafe { String::from_utf8_unchecked(bytes.to_vec()) };
        Ok(Self { inner: string })
    }

    /// Converts a vector of bytes to a `HeaderValue` without checking that the string contains
    /// valid ASCII.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check that the bytes passed to it are valid
    /// ASCII. If this constraint is violated, it may cause memory
    /// unsafety issues with future users of the HeaderValue, as the rest of the library assumes
    /// that Strings are valid ASCII.
    pub unsafe fn from_ascii_unchecked(bytes: Vec<u8>) -> Self {
        let string = String::from_utf8_unchecked(bytes);
        Self { inner: string }
    }

    /// Get the header value as a `&str`
    pub fn as_str(&self) -> &str {
        &self.inner
    }
}

impl From<Mime> for HeaderValue {
    fn from(mime: Mime) -> Self {
        HeaderValue {
            inner: format!("{}", mime),
        }
    }
}

impl From<Cookie<'_>> for HeaderValue {
    fn from(cookie: Cookie<'_>) -> Self {
        HeaderValue {
            inner: cookie.to_string(),
        }
    }
}

impl From<&Mime> for HeaderValue {
    fn from(mime: &Mime) -> Self {
        HeaderValue {
            inner: format!("{}", mime),
        }
    }
}

impl FromStr for HeaderValue {
    type Err = Error;

    /// Create a new `HeaderValue`.
    ///
    /// This checks it's valid ASCII.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(Error::from(ErrorKind::InvalidData));
        }
        Ok(Self {
            inner: String::from(s),
        })
    }
}

impl Display for HeaderValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}
