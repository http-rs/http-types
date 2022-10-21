use crate::headers::{Header, HeaderName, HeaderValue, Headers, EXPIRES};
use crate::utils::{fmt_http_date, parse_http_date};

use std::fmt::Debug;
use std::time::{Duration, SystemTime};

/// HTTP `Expires` header
///
/// # Specifications
///
/// - [RFC 7234, section 5.3: Expires](https://tools.ietf.org/html/rfc7234#section-5.3)
///
/// # Examples
///
/// ```
/// # fn main() -> anyhow::Result<()> {
/// #
/// use http_types::Response;
/// use http_types::cache::Expires;
/// use std::time::{SystemTime, Duration};
///
/// let time = SystemTime::now() + Duration::from_secs(5 * 60);
/// let expires = Expires::new_at(time);
///
/// let mut res = Response::new(200);
/// res.insert_header(&expires, &expires);
///
/// let expires = Expires::from_headers(res)?.unwrap();
///
/// // HTTP dates only have second-precision.
/// let elapsed = time.duration_since(expires.expiration())?;
/// assert_eq!(elapsed.as_secs(), 0);
/// #
/// # Ok(()) }
/// ```
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Expires {
    instant: SystemTime,
}

impl Expires {
    /// Create a new instance of `Expires`.
    pub fn new(dur: Duration) -> Self {
        let instant = SystemTime::now() + dur;
        Self { instant }
    }

    /// Create a new instance of `Expires` from secs.
    pub fn new_at(instant: SystemTime) -> Self {
        Self { instant }
    }

    /// Get the expiration time.
    pub fn expiration(&self) -> SystemTime {
        self.instant
    }

    /// Create an instance of `Expires` from a `Headers` instance.
    pub fn from_headers(headers: impl AsRef<Headers>) -> crate::Result<Option<Self>> {
        let headers = match headers.as_ref().get(EXPIRES) {
            Some(headers) => headers,
            None => return Ok(None),
        };

        // If we successfully parsed the header then there's always at least one
        // entry. We want the last entry.
        let header = headers.iter().last().unwrap();

        let instant = parse_http_date(header.as_str())?;
        Ok(Some(Self { instant }))
    }
}

impl Header for Expires {
    fn header_name(&self) -> HeaderName {
        EXPIRES
    }
    fn header_value(&self) -> HeaderValue {
        let output = fmt_http_date(self.instant);

        // SAFETY: the internal string is validated to be ASCII.
        unsafe { HeaderValue::from_bytes_unchecked(output.into()) }
    }
}

#[cfg(test)]
mod test {
    use crate::headers::Headers;
    use crate::StatusCode;

    use super::*;

    #[test]
    fn smoke() -> anyhow::Result<()> {
        let time = SystemTime::now() + Duration::from_secs(5 * 60);
        let expires = Expires::new_at(time);

        let mut headers = Headers::new();
        expires.apply_header(&mut headers);

        let expires = Expires::from_headers(headers)?.unwrap();

        // HTTP dates only have second-precision
        let elapsed = time.duration_since(expires.expiration())?;
        assert_eq!(elapsed.as_secs(), 0);
        Ok(())
    }

    #[test]
    fn bad_request_on_parse_error() {
        let mut headers = Headers::new();
        headers.insert(EXPIRES, "<nori ate the tag. yum.>").unwrap();
        let err = Expires::from_headers(headers).unwrap_err();
        assert_eq!(err.associated_status_code(), Some(StatusCode::BadRequest));
    }
}
