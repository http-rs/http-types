use crate::range::bytes;
use crate::{
    headers::{HeaderName, HeaderValue, Headers, ToHeaderValues, ACCEPT_RANGES},
    Error,
};

use std::fmt::{self, Debug, Display};
use std::option;

/// HTTP Accept-Ranges response header.
///
/// Accept-Ranges header indicates that the server supports
/// range requests and specifies the unit to be used by
/// clients for range requests.
///
/// The default value is to not accept range requests.
///
/// # Specifications
///
/// - [RFC 7233, section 2.3: Accept-Ranges](https://tools.ietf.org/html/rfc7233#section-2.3)
/// - [RFC 7233, section 2.1: Byte Ranges](https://tools.ietf.org/html/rfc7233#section-2.1)
/// - [IANA HTTP parameters, range-units: HTTP Range Unit Registry](https://www.iana.org/assignments/http-parameters/http-parameters.xhtml)
///
/// # Examples
///
/// Accepting ranges specified in byte unit (the widely used default):
///
/// ```
/// # fn main() -> http_types::Result<()> {
/// #
/// use http_types::range::AcceptRanges;
/// use http_types::Response;
///
/// let accept_ranges = AcceptRanges::Bytes;
///
/// let mut res = Response::new(200);
/// accept_ranges.apply(&mut res);
///
/// let accept_ranges = AcceptRanges::from_headers(res)?.unwrap();
/// assert_eq!(accept_ranges, AcceptRanges::Bytes);
/// #
/// # Ok(()) }
/// ```
///
/// Range requests not accepted:
///
/// ```
/// # fn main() -> http_types::Result<()> {
/// #
/// use http_types::range::AcceptRanges;
/// use http_types::Response;
///
/// let accept_ranges = AcceptRanges::None;
///
/// let mut res = Response::new(200);
/// accept_ranges.apply(&mut res);
///
/// let accept_ranges = AcceptRanges::from_headers(res)?.unwrap();
/// assert_eq!(accept_ranges, AcceptRanges::None);
/// #
/// # Ok(()) }
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum AcceptRanges {
    /// Accepts bytes based range requests.
    Bytes,
    /// Do not accept range requests.
    None,
}

impl AcceptRanges {
    /// The "none" value used when range requests are not accepted.
    const NONE: &'static str = "none";

    /// Create a new instance from headers.
    ///
    /// Only a single AcceptRanges per resource is assumed to exist. If multiple Accept-Ranges
    /// headers are found the last one is used.
    pub fn from_headers(headers: impl AsRef<Headers>) -> crate::Result<Option<Self>> {
        let headers = match headers.as_ref().get(ACCEPT_RANGES) {
            Some(headers) => headers,
            None => return Ok(None),
        };

        // If a header is returned we can assume at least one exists.
        let s = headers.iter().last().unwrap().as_str();
        Self::from_str(s).map(Some)
    }

    /// Create an AcceptRanges from a string.
    pub(crate) fn from_str(s: &str) -> crate::Result<Self> {
        match s {
            Self::NONE => Ok(AcceptRanges::None),
            bytes::ACCEPT_RANGE_VALUE => Ok(AcceptRanges::Bytes),
            _ => Err(Error::new_adhoc("unknown Accept-Ranges header")),
        }
    }

    /// Sets the `Accept-Ranges` header.
    pub fn apply(&self, mut headers: impl AsMut<Headers>) {
        headers.as_mut().insert(ACCEPT_RANGES, self.value());
    }

    /// Get the `HeaderName`.
    pub fn name(&self) -> HeaderName {
        ACCEPT_RANGES
    }

    /// Get the `HeaderValue`.
    pub fn value(&self) -> HeaderValue {
        let s = match self {
            AcceptRanges::Bytes => bytes::ACCEPT_RANGE_VALUE,
            AcceptRanges::None => AcceptRanges::NONE,
        };
        // SAFETY: the internal string is validated to be ASCII.
        unsafe { HeaderValue::from_bytes_unchecked(s.into()) }
    }
}

impl Display for AcceptRanges {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AcceptRanges::Bytes => write!(f, "{}", bytes::ACCEPT_RANGE_VALUE),
            AcceptRanges::None => write!(f, "{}", AcceptRanges::NONE),
        }
    }
}

impl ToHeaderValues for AcceptRanges {
    type Iter = option::IntoIter<HeaderValue>;
    fn to_header_values(&self) -> crate::Result<Self::Iter> {
        // A HeaderValue will always convert into itself.
        Ok(self.value().to_header_values().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::headers::Headers;

    use crate::Response;

    #[test]
    fn accept_ranges_none() -> crate::Result<()> {
        let mut headers = Headers::new();
        headers.insert(ACCEPT_RANGES, "none");
        let accept_ranges = AcceptRanges::from_headers(headers).unwrap().unwrap();
        assert_eq!(accept_ranges, AcceptRanges::None);

        let accept_ranges = AcceptRanges::None;
        let mut res = Response::new(200);
        accept_ranges.apply(&mut res);

        let raw_header_value = res.header(ACCEPT_RANGES).unwrap();
        assert_eq!(raw_header_value, "none");

        Ok(())
    }

    #[test]
    fn accept_ranges_bytes() -> crate::Result<()> {
        let mut headers = Headers::new();
        headers.insert(ACCEPT_RANGES, "bytes");
        let accept_ranges = AcceptRanges::from_headers(headers).unwrap().unwrap();
        assert_eq!(accept_ranges, AcceptRanges::Bytes);

        let accept_ranges = AcceptRanges::Bytes;
        let mut res = Response::new(200);
        accept_ranges.apply(&mut res);

        let raw_header_value = res.header(ACCEPT_RANGES).unwrap();
        assert_eq!(raw_header_value, "bytes");

        Ok(())
    }
}
