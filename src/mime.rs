//! IANA Media Types.
//!
//! [Read more](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types).

use crate::headers::ParseError;
use std::fmt::{self, Display};
use std::str::FromStr;

/// An IANA media type.
#[derive(Debug)]
pub struct Mime {
    /// The inner representation of the string.
    pub(crate) string: String,
    /// A const-friendly string. Useful because `String::from` cannot be used in const contexts.
    pub(crate) static_str: Option<&'static str>,
}

impl Display for Mime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(string) = self.static_str {
            write!(f, "{}", string)
        } else {
            write!(f, "{}", self.string)
        }
    }
}

impl FromStr for Mime {
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

/// Content-Type for JavaScript.
pub const JAVASCRIPT: Mime = Mime {
    static_str: Some("application/javascript; charset=utf-8"),
    string: String::new(),
};

/// Content-Type for JSON.
pub const JSON: Mime = Mime {
    static_str: Some("application/json"),
    string: String::new(),
};

/// Content-Type for CSS.
pub const CSS: Mime = Mime {
    static_str: Some("text/css; charset=utf-8"),
    string: String::new(),
};

/// Content-Type for HTML.
pub const HTML: Mime = Mime {
    static_str: Some("text/html; charset=utf-8"),
    string: String::new(),
};

/// Content-Type for Server Sent Events
pub const SSE: Mime = Mime {
    static_str: Some("text/event-stream"),
    string: String::new(),
};

/// Content-Type for plain text.
pub const PLAIN: Mime = Mime {
    static_str: Some("text/plain; charset=utf-8"),
    string: String::new(),
};

/// Content-Type for byte streams.
pub const BYTE_STREAM: Mime = Mime {
    static_str: Some("application/octet-stream"),
    string: String::new(),
};

/// Content-Type for form.
pub const FORM: Mime = Mime {
    static_str: Some("application/x-www-urlencoded"),
    string: String::new(),
};

/// Content-Type for a multipart form.
pub const MULTIPART_FORM: Mime = Mime {
    static_str: Some("multipart/form-data"),
    string: String::new(),
};
