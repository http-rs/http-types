use crate::headers::{HeaderName, HeaderValue, Headers, ToHeaderValues, ETAG};
use crate::{Error, StatusCode};

use std::fmt::Debug;
use std::option;

/// HTTP Entity Tags.
///
/// ETags provide an ID for a particular resource, enabling clients and servers
/// to reason about caches and make conditional requests.
///
/// # Specifications
///
/// - [RFC 7232 HTTP/1.1: Conditional Requests](https://tools.ietf.org/html/rfc7232#section-2.3)
#[derive(Debug)]
pub enum Etag {
    /// An Etag using strong validation.
    Strong(String),
    /// An ETag using weak validation.
    Weak(String),
}

impl Etag {
    /// Create a new Etag that uses strong validation.
    pub fn new(s: String) -> Self {
        debug_assert!(!s.contains('\\'), "ETags ought to avoid backslash chars");
        Self::Strong(s)
    }

    /// Create a new Etag that uses weak validation.
    pub fn new_weak(s: String) -> Self {
        debug_assert!(!s.contains('\\'), "ETags ought to avoid backslash chars");
        Self::Weak(s)
    }

    /// Create a new instance from headers.
    ///
    /// Only a single ETag per resource is assumed to exist. If multiple ETag
    /// headers are found the last one is used.
    pub fn from_headers(headers: impl AsRef<Headers>) -> crate::Result<Option<Self>> {
        let headers = match headers.as_ref().get(ETAG) {
            Some(headers) => headers,
            None => return Ok(None),
        };

        // If a header is returned we can assume at least one exists.
        let mut s = headers.iter().last().unwrap().as_str();

        let weak = if s.starts_with("/W") {
            s = &s[2..];
            true
        } else {
            false
        };

        let s = match s.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
            Some(s) => s.to_owned(),
            None => {
                return Err(Error::from_str(
                    StatusCode::BadRequest,
                    "Invalid ETag header",
                ))
            }
        };

        let etag = if weak { Self::Weak(s) } else { Self::Strong(s) };
        Ok(Some(etag))
    }

    /// Sets the `ETag` header.
    pub fn apply(&self, mut headers: impl AsMut<Headers>) {
        headers.as_mut().insert(ETAG, self.value());
    }

    /// Get the `HeaderName`.
    pub fn name(&self) -> HeaderName {
        ETAG
    }

    /// Get the `HeaderValue`.
    pub fn value(&self) -> HeaderValue {
        let s = match self {
            Self::Strong(s) => format!(r#""{}""#, s),
            Self::Weak(s) => format!(r#"W/"{}""#, s),
        };
        // SAFETY: the internal string is validated to be ASCII.
        unsafe { HeaderValue::from_bytes_unchecked(s.into()) }
    }
}

impl ToHeaderValues for Etag {
    type Iter = option::IntoIter<HeaderValue>;
    fn to_header_values(&self) -> crate::Result<Self::Iter> {
        // A HeaderValue will always convert into itself.
        Ok(self.value().to_header_values().unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::headers::{Headers, CACHE_CONTROL};

    #[test]
    fn smoke() -> crate::Result<()> {
        let mut etag = Etag::new("0xcafebeef");

        let mut headers = Headers::new();
        entries.apply(&mut headers);

        let entries = Etag::from_headers(headers)?.unwrap();
        let mut entries = entries.iter();
        assert_eq!(entries.next().unwrap(), &CacheDirective::Immutable);
        assert_eq!(entries.next().unwrap(), &CacheDirective::NoStore);
        Ok(())
    }

    #[test]
    fn ignore_unkonwn_directives() -> crate::Result<()> {
        let mut headers = Headers::new();
        headers.insert(CACHE_CONTROL, "barrel_roll");
        let entries = Etag::from_headers(headers)?.unwrap();
        let mut entries = entries.iter();
        assert!(entries.next().is_none());
        Ok(())
    }

    #[test]
    fn bad_request_on_parse_error() -> crate::Result<()> {
        let mut headers = Headers::new();
        headers.insert(CACHE_CONTROL, "min-fresh=0.9"); // floats are not supported
        let err = Etag::from_headers(headers).unwrap_err();
        assert_eq!(err.status(), 400);
        Ok(())
    }
}
