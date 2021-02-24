use crate::headers::{Header, HeaderName, HeaderValue, Headers, AGE};
use crate::Status;

use std::fmt::Debug;

use std::time::Duration;

/// HTTP `Age` header
///
/// # Specifications
///
/// - [RFC 7234, section 5.1: Age](https://tools.ietf.org/html/rfc7234#section-5.1)
///
/// # Examples
///
/// ```
/// # fn main() -> http_types::Result<()> {
/// #
/// use http_types::Response;
/// use http_types::cache::Age;
///
/// let age = Age::from_secs(12);
///
/// let mut res = Response::new(200);
/// res.insert_header(&age, &age);
///
/// let age = Age::from_headers(res)?.unwrap();
/// assert_eq!(age, Age::from_secs(12));
/// #
/// # Ok(()) }
/// ```
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Age {
    dur: Duration,
}

impl Age {
    /// Create a new instance of `Age`.
    pub fn new(dur: Duration) -> Self {
        Self { dur }
    }

    /// Create a new instance of `Age` from secs.
    pub fn from_secs(secs: u64) -> Self {
        let dur = Duration::from_secs(secs);
        Self { dur }
    }

    /// Get the duration from the header.
    pub fn duration(&self) -> Duration {
        self.dur
    }

    /// Create an instance of `Age` from a `Headers` instance.
    pub fn from_headers(headers: impl AsRef<Headers>) -> crate::Result<Option<Self>> {
        let headers = match headers.as_ref().get(AGE) {
            Some(headers) => headers,
            None => return Ok(None),
        };

        // If we successfully parsed the header then there's always at least one
        // entry. We want the last entry.
        let header = headers.iter().last().unwrap();

        let num: u64 = header.as_str().parse().status(400)?;
        let dur = Duration::from_secs_f64(num as f64);

        Ok(Some(Self { dur }))
    }
}

impl Header for Age {
    fn header_name(&self) -> HeaderName {
        AGE
    }

    fn header_value(&self) -> HeaderValue {
        let output = self.dur.as_secs().to_string();

        // SAFETY: the internal string is validated to be ASCII.
        unsafe { HeaderValue::from_bytes_unchecked(output.into()) }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::headers::Headers;

    #[test]
    fn smoke() -> crate::Result<()> {
        let age = Age::new(Duration::from_secs(12));

        let mut headers = Headers::new();
        age.apply_header(&mut headers);

        let age = Age::from_headers(headers)?.unwrap();
        assert_eq!(age, Age::new(Duration::from_secs(12)));
        Ok(())
    }

    #[test]
    fn bad_request_on_parse_error() {
        let mut headers = Headers::new();
        headers.insert(AGE, "<nori ate the tag. yum.>");
        let err = Age::from_headers(headers).unwrap_err();
        assert_eq!(err.status(), 400);
    }
}
