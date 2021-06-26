use crate::headers::{HeaderName, HeaderValue, Headers, ToHeaderValues, RANGE};
use crate::range::{bytes, BytesRangeSet};
use crate::{Error, StatusCode};

use std::fmt::{self, Debug, Display};
use std::option;

/// HTTP Range request header.
///
/// Range header in a GET request modifies the method
/// semantics to request transfer of only one or more subranges of the
/// selected representation data, rather than the entire selected
/// representation data.
///
/// # Specifications
///
/// - [RFC 7233, section 3.1: Range](https://tools.ietf.org/html/rfc7233#section-3.1)
/// - [RFC 7233, Appendix D: Collected ABNF](https://tools.ietf.org/html/rfc7233#appendix-D)
/// - [IANA HTTP parameters, range-units: HTTP Range Unit Registry](https://www.iana.org/assignments/http-parameters/http-parameters.xhtml#range-units)
///
/// # Examples
///
/// Range header using bytes:
///
/// ```
/// # fn main() -> http_types::Result<()> {
/// #
/// use http_types::range::{BytesRangeSet, Range};
/// use http_types::{Method, Request, Url};
///
/// let mut byte_range_set = BytesRangeSet::new();
/// byte_range_set.push(0, 500);
///
/// let range = Range::Bytes(byte_range_set.clone());
/// let mut req = Request::new(Method::Get, Url::parse("http://example.com").unwrap());
/// range.apply(&mut req);
///
/// let range = Range::from_headers(req)?.unwrap();
/// assert_eq!(range, Range::Bytes(byte_range_set));
/// #
/// # Ok(()) }
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum Range {
    /// Bytes based range requests.
    Bytes(BytesRangeSet),
}

impl Range {
    /// Create a new instance from a Range headers.
    ///
    /// Only a single Range per resource is assumed to exist. If multiple Range
    /// headers are found the last one is used.
    pub fn from_headers(headers: impl AsRef<Headers>) -> crate::Result<Option<Self>> {
        let headers = match headers.as_ref().get(RANGE) {
            Some(headers) => headers,
            None => return Ok(None),
        };

        // If a header is returned we can assume at least one exists.
        let s = headers.iter().last().unwrap().as_str();

        Self::from_str(s).map(Some)
    }

    /// Create a ByteRanges from a string.
    pub(crate) fn from_str(s: &str) -> crate::Result<Self> {
        let fn_err = || {
            Error::from_str(
                StatusCode::BadRequest,
                "Invalid Range header for byte ranges",
            )
        };

        match s {
            s if s.starts_with(bytes::RANGE_PREFIX) => {
                let s = &s[bytes::RANGE_PREFIX.len()..];
                BytesRangeSet::from_str(s).map(Range::Bytes)
            }
            _ => Err(fn_err()),
        }
    }

    /// Sets the `Range` header.
    pub fn apply(&self, mut headers: impl AsMut<Headers>) {
        headers.as_mut().insert(RANGE, self.value());
    }

    /// Get the `HeaderName`.
    pub fn name(&self) -> HeaderName {
        RANGE
    }

    /// Get the `HeaderValue`.
    pub fn value(&self) -> HeaderValue {
        let s = self.to_string();
        // SAFETY: the internal string is validated to be ASCII.
        unsafe { HeaderValue::from_bytes_unchecked(s.into()) }
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Range::Bytes(ref bytes_range) => {
                write!(f, "{}{}", bytes::RANGE_PREFIX, bytes_range)
            }
        }
    }
}

impl ToHeaderValues for Range {
    type Iter = option::IntoIter<HeaderValue>;
    fn to_header_values(&self) -> crate::Result<Self::Iter> {
        // A HeaderValue will always convert into itself.
        Ok(self.value().to_header_values().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::headers::RANGE;
    use crate::range::{BytesRange, BytesRangeSet};
    use crate::{Method, Request, Url};

    #[test]
    fn bytes_range() -> crate::Result<()> {
        let mut req = Request::new(Method::Get, Url::parse("http://example.com").unwrap());
        req.insert_header(RANGE, "bytes=1-5");
        let range = Range::from_headers(req)?.unwrap();
        match range {
            Range::Bytes(bytes_range_set) => {
                assert_eq!(bytes_range_set.len(), 1);
                assert_eq!(bytes_range_set.first(), Some(BytesRange::new(1, 5)));
            }
        }

        Ok(())
    }

    #[test]
    fn bytes_range_apply() -> crate::Result<()> {
        let mut ranges = BytesRangeSet::new();
        ranges.push(1, 5);
        ranges.push(None, 5);
        let range = Range::Bytes(ranges);
        let mut req = Request::new(Method::Get, Url::parse("http://example.com").unwrap());
        range.apply(&mut req);
        assert_eq!(req[RANGE], "bytes=1-5,-5");
        Ok(())
    }

    #[test]
    fn invalid_unit() {
        let mut req = Request::new(Method::Get, Url::parse("http://example.com").unwrap());
        req.insert_header(RANGE, "foo=1-5");
        let err = Range::from_headers(req).unwrap_err();
        assert_eq!(err.status(), StatusCode::BadRequest);
    }
}
