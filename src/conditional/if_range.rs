use crate::conditional::ETag;
use crate::headers::{HeaderName, HeaderValue, Headers, ToHeaderValues, IF_RANGE};
use crate::utils::{fmt_http_date, parse_http_date};

use std::fmt::{self, Display};
use std::option;
use std::time::SystemTime;

/// Apply the HTTP method if the ETag or date matches
///
/// # Specifications
///
/// - [RFC 7233, section 3.2: Range](https://tools.ietf.org/html/rfc7233#section-3.2)
/// - [RFC 7232, section 2.3: ETag](https://tools.ietf.org/html/rfc7232#section-2.3)
/// - [RFC 7231, section 7.1.1.1: Date/Time Formats](https://tools.ietf.org/html/rfc7231#section-7.1.1.1)
///
/// # Examples
///
/// If-Range with strong ETag:
///
/// ```
/// # fn main() -> http_types::Result<()> {
/// #
/// use http_types::conditional::{ETag, IfRange};
/// use http_types::Response;
///
/// let strong_etag = ETag::new(String::from("aBcdEFghijkl"));
/// let ifrange = IfRange::from(strong_etag.clone());
///
/// let mut res = Response::new(200);
/// ifrange.apply(&mut res);
///
/// let ifrange = IfRange::from_headers(res)?.unwrap();
/// assert_eq!(ifrange, IfRange::ETag(strong_etag));
/// #
/// # Ok(()) }
/// ```
///
/// If-Range with HTTP Date:
///
/// ```
/// # fn main() -> http_types::Result<()> {
/// #
/// use http_types::conditional::IfRange;
/// use http_types::{Error, Response, StatusCode};
/// use std::time::{Duration, SystemTime};
///
/// let time = SystemTime::now() + Duration::from_secs(5 * 60);
/// let ifrange = IfRange::from(time);
///
/// let mut res = Response::new(200);
/// ifrange.apply(&mut res);
///
/// let ifrange = IfRange::from_headers(res)?.unwrap();
///
/// let response_time = match ifrange {
///     IfRange::Date(d) => d,
///     _ => {
///         return Err(Error::from_str(
///             StatusCode::RequestedRangeNotSatisfiable,
///             "Invalid If-Range header",
///         ))
///     }
/// };
///
/// // HTTP dates only have second-precision.
/// let elapsed = time.duration_since(response_time)?;
/// assert_eq!(elapsed.as_secs(), 0);
/// #
/// # Ok(()) }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IfRange {
    /// If-Range condition expressed with an `ETag`.
    ETag(ETag),
    /// If-Range condition expressed with an `HTTP Date`.
    Date(SystemTime),
}

impl IfRange {
    /// Create an instance of `IfRange` from a `Headers` instance.
    pub fn from_headers(headers: impl AsRef<Headers>) -> crate::Result<Option<Self>> {
        let headers = match headers.as_ref().get(IF_RANGE) {
            Some(headers) => headers,
            None => return Ok(None),
        };

        // If we successfully parsed the header then there's always at least one
        // entry. We want the last entry.
        let header = headers.iter().last().unwrap();
        let header_str = header.as_str();

        parse_http_date(header_str)
            .map(|d| Some(IfRange::from(d)))
            .or_else(|_| Ok(Some(IfRange::from(ETag::from_str(header_str)?))))
    }

    /// Insert a `HeaderName` + `HeaderValue` pair into a `Headers` instance.
    pub fn apply(&self, mut headers: impl AsMut<Headers>) {
        headers.as_mut().insert(IF_RANGE, self.value());
    }

    /// Get the `HeaderName`.
    pub fn name(&self) -> HeaderName {
        IF_RANGE
    }

    /// Get the `HeaderValue`.
    pub fn value(&self) -> HeaderValue {
        let output = self.to_string();

        // SAFETY: the internal string is validated to be ASCII.
        unsafe { HeaderValue::from_bytes_unchecked(output.into()) }
    }
}

impl Display for IfRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            IfRange::ETag(t) => t.to_string(),
            IfRange::Date(d) => fmt_http_date(*d),
        };
        write!(f, "{}", s)
    }
}

impl ToHeaderValues for IfRange {
    type Iter = option::IntoIter<HeaderValue>;
    fn to_header_values(&self) -> crate::Result<Self::Iter> {
        // A HeaderValue will always convert into itself.
        Ok(self.value().to_header_values().unwrap())
    }
}

impl From<ETag> for IfRange {
    fn from(etag: ETag) -> Self {
        IfRange::ETag(etag)
    }
}

impl From<SystemTime> for IfRange {
    fn from(time: SystemTime) -> Self {
        IfRange::Date(time)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::headers::Headers;
    use crate::{Error, StatusCode};

    use std::time::Duration;

    #[test]
    fn if_range_time() -> crate::Result<()> {
        let time = SystemTime::now() + Duration::from_secs(5 * 60);
        let if_range = IfRange::from(time);

        let mut headers = Headers::new();
        if_range.apply(&mut headers);

        let if_range = IfRange::from_headers(headers)?.unwrap();

        let header_time = match if_range {
            IfRange::Date(d) => d,
            _ => {
                return Err(Error::from_str(
                    StatusCode::RequestedRangeNotSatisfiable,
                    "Invalid If-Range header",
                ))
            }
        };

        // HTTP dates only have second-precision
        let elapsed = time.duration_since(header_time)?;
        assert_eq!(elapsed.as_secs(), 0);
        Ok(())
    }

    #[test]
    fn if_range_etag() -> crate::Result<()> {
        let etag = ETag::new(String::from("foobar"));
        let if_range = IfRange::from(etag.clone());

        let mut headers = Headers::new();
        if_range.apply(&mut headers);

        let if_range = IfRange::from_headers(headers)?.unwrap();

        let header_etag = match if_range {
            IfRange::ETag(t) => t,
            _ => {
                return Err(Error::from_str(
                    StatusCode::RequestedRangeNotSatisfiable,
                    "Invalid If-Range header",
                ))
            }
        };

        assert_eq!(header_etag, etag);
        Ok(())
    }

    #[test]
    fn bad_request_on_parse_error() -> crate::Result<()> {
        let mut headers = Headers::new();
        headers.insert(IF_RANGE, "<nori ate the tag. yum.>");
        let err = IfRange::from_headers(headers).unwrap_err();
        assert_eq!(err.status(), 400);
        Ok(())
    }
}
