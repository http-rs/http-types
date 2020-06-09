use crate::headers::{HeaderName, HeaderValue, Headers, ToHeaderValues, CONTENT_RANGE};
use crate::range::{bytes, BytesContentRange};
use crate::{Error, StatusCode};

use std::fmt::{self, Debug, Display};
use std::option;

/// HTTP ContentRange response header.
///
/// The "Content-Range" header field is sent in a single part 206
/// (Partial Content) response to indicate the partial range of the
/// selected representation enclosed as the message payload, sent in each
/// part of a multipart 206 response to indicate the range enclosed
/// within each body part, and sent in 416 (Range Not Satisfiable)
/// responses to provide information about the selected representation.
///
/// # Specifications
///
/// - [RFC 7233, section 4.2: Range](https://tools.ietf.org/html/rfc7233#section-4.2)
///
/// # Examples
///
/// Encoding a Content-Range header for byte range 1-5 of a 10 bytes size document:
///
/// ```
/// # fn main() -> http_types::Result<()> {
/// #
/// use http_types::range::{BytesContentRange, BytesRange, ContentRange};
/// use http_types::{Response, StatusCode};
///
/// let bytes_content_range = BytesContentRange::new().with_range(1, 5).with_size(10);
/// let content_range = ContentRange::Bytes(bytes_content_range);
///
/// let mut res = Response::new(StatusCode::PartialContent);
/// content_range.apply(&mut res);
///
/// let content_range = ContentRange::from_headers(res)?.unwrap();
/// if let ContentRange::Bytes(bytes_content_range) = content_range {
///     assert_eq!(bytes_content_range.range(), Some(BytesRange::new(1, 5)));
///     assert_eq!(bytes_content_range.size(), Some(10));
/// }
/// #
/// # Ok(()) }
/// ```
///
/// Encoding a Content-Range header for byte range 1-5 with unknown size:
///
/// ```
/// # fn main() -> http_types::Result<()> {
/// #
/// use http_types::range::{BytesContentRange, BytesRange, ContentRange};
/// use http_types::{Response, StatusCode};
///
/// let mut bytes_content_range = BytesContentRange::new().with_range(1, 5);
/// let content_range = ContentRange::Bytes(bytes_content_range);
///
/// let mut res = Response::new(StatusCode::PartialContent);
/// content_range.apply(&mut res);
///
/// let content_range = ContentRange::from_headers(res)?.unwrap();
/// if let ContentRange::Bytes(bytes_content_range) = content_range {
///     assert_eq!(bytes_content_range.range(), Some(BytesRange::new(1, 5)));
///     assert_eq!(bytes_content_range.size(), None);
/// }
/// #
/// # Ok(()) }
/// ```
///
/// Responding to an invalid range request for a 10 bytes document:
///
/// ```
/// # fn main() -> http_types::Result<()> {
/// #
/// use http_types::range::{BytesContentRange, BytesRange, ContentRange};
/// use http_types::{Response, StatusCode};
///
/// let bytes_content_range = BytesContentRange::new().with_size(10);
/// let content_range = ContentRange::Bytes(bytes_content_range);
///
/// let mut res = Response::new(StatusCode::RequestedRangeNotSatisfiable);
/// content_range.apply(&mut res);
///
/// let content_range = ContentRange::from_headers(res)?.unwrap();
/// if let ContentRange::Bytes(bytes_content_range) = content_range {
///     assert_eq!(bytes_content_range.range(), None);
///     assert_eq!(bytes_content_range.size(), Some(10));
/// }
/// #
/// # Ok(()) }
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ContentRange {
    /// Bytes based content range header.
    Bytes(BytesContentRange),
}

impl ContentRange {
    /// Create a new instance from a Content-Range headers.
    ///
    /// Only a single Content-Range per resource is assumed to exist. If multiple Range
    /// headers are found the last one is used.
    pub fn from_headers(headers: impl AsRef<Headers>) -> crate::Result<Option<Self>> {
        let headers = match headers.as_ref().get(CONTENT_RANGE) {
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
                StatusCode::RequestedRangeNotSatisfiable,
                "Invalid Content-Range value",
            )
        };

        match s {
            s if s.starts_with(bytes::CONTENT_RANGE_PREFIX) => {
                let s = &s[bytes::CONTENT_RANGE_PREFIX.len()..];
                BytesContentRange::from_str(s).map(ContentRange::Bytes)
            }
            _ => Err(fn_err()),
        }
    }

    /// Sets the `Range` header.
    pub fn apply(&self, mut headers: impl AsMut<Headers>) {
        headers.as_mut().insert(CONTENT_RANGE, self.value());
    }

    /// Get the `HeaderName`.
    pub fn name(&self) -> HeaderName {
        CONTENT_RANGE
    }

    /// Get the `HeaderValue`.
    pub fn value(&self) -> HeaderValue {
        let s = self.to_string();
        // SAFETY: the internal string is validated to be ASCII.
        unsafe { HeaderValue::from_bytes_unchecked(s.into()) }
    }
}

impl Display for ContentRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ContentRange::Bytes(ref bytes_content_range) => {
                write!(f, "{}{}", bytes::CONTENT_RANGE_PREFIX, bytes_content_range)
            }
        }
    }
}

impl ToHeaderValues for ContentRange {
    type Iter = option::IntoIter<HeaderValue>;
    fn to_header_values(&self) -> crate::Result<Self::Iter> {
        // A HeaderValue will always convert into itself.
        Ok(self.value().to_header_values().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::headers::CONTENT_RANGE;
    use crate::range::BytesRange;
    use crate::{Response, StatusCode};

    #[test]
    fn bytes_content_range() -> crate::Result<()> {
        let mut res = Response::new(StatusCode::PartialContent);
        res.insert_header(CONTENT_RANGE, "bytes 1-5/100");
        let content_range = ContentRange::from_headers(res)?.unwrap();

        match content_range {
            ContentRange::Bytes(bytes_content_range) => {
                assert_eq!(bytes_content_range.range(), Some(BytesRange::new(1, 5)));
                assert_eq!(bytes_content_range.size(), Some(100));
            }
        }

        Ok(())
    }

    #[test]
    fn bytes_content_range_apply() -> crate::Result<()> {
        let bytes_content_range = BytesContentRange::new().with_range(1, 5).with_size(10);
        let content_range = ContentRange::Bytes(bytes_content_range);
        let mut res = Response::new(StatusCode::PartialContent);
        content_range.apply(&mut res);
        assert_eq!(res[CONTENT_RANGE], "bytes 1-5/10");
        Ok(())
    }

    #[test]
    fn invalid_unit() {
        let mut res = Response::new(StatusCode::PartialContent);
        res.insert_header(CONTENT_RANGE, "foo 1-5/*");
        let err = ContentRange::from_headers(res).unwrap_err();
        assert_eq!(err.status(), StatusCode::RequestedRangeNotSatisfiable);
    }
}
