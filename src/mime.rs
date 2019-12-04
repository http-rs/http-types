//! IANA Media Types.
//!
//! [Read more](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types).

use std::fmt::{self, Display};

/// An IANA media type.
#[derive(Debug)]
pub struct Mime {
    inner: &'static str,
}

impl Mime {
    /// Create a new instance.
    pub const fn new(mime: &'static str) -> Self {
        if !mime.is_ascii() {
            panic!("mime must be valid ascii -- this should be an error not a panic oops");
        }
        Self { inner: mime }
    }
}

impl Display for Mime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

/// Content-Type for JavaScript.
pub const JAVASCRIPT: Mime = Mime::new("application/javascript; charset=utf-8");

/// Content-Type for JSON.
pub const JSON: Mime = Mime::new("application/json");

/// Content-Type for CSS.
pub const CSS: Mime = Mime::new("text/css; charset=utf-8");

/// Content-Type for HTML.
pub const HTML: Mime = Mime::new("text/html; charset=utf-8");

/// Content-Type for Server Sent Events
pub const SSE: Mime = Mime::new("text/event-stream");

/// Content-Type for plain text.
pub const PLAIN: Mime = Mime::new("text/plain; charset=utf-8");

/// Content-Type for byte streams.
pub const BYTE_STREAM: Mime = Mime::new("application/octet-stream");

/// Content-Type for form.
pub const FORM: Mime = Mime::new("application/x-www-urlencoded");

/// Content-Type for a multipart form.
pub const MULTIPART_FORM: Mime = Mime::new("multipart/form-data");
