use crate::headers::{HeaderName, HeaderValue, Headers, EXPECT};
use crate::{ensure_eq_status, headers::Header};

use std::fmt::Debug;

/// HTTP `Expect` header
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Expect)
///
/// # Specifications
///
/// - [RFC 7231, section 5.1.1: Expect](https://tools.ietf.org/html/rfc7231#section-5.1.1)
///
/// # Examples
///
/// ```
/// # fn main() -> http_types::Result<()> {
/// #
/// use http_types::Response;
/// use http_types::other::Expect;
///
/// let expect = Expect::new();
///
/// let mut res = Response::new(200);
/// res.insert_header(&expect, &expect);
///
/// let expect = Expect::from_headers(res)?.unwrap();
/// assert_eq!(expect, Expect::new());
/// #
/// # Ok(()) }
/// ```
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Expect {
    _priv: (),
}

impl Expect {
    /// Create a new instance of `Expect`.
    #[must_use]
    pub fn new() -> Self {
        Self { _priv: () }
    }

    /// Create an instance of `Expect` from a `Headers` instance.
    pub fn from_headers(headers: impl AsRef<Headers>) -> crate::Result<Option<Self>> {
        let Some(headers) = headers.as_ref().get(EXPECT) else { return Ok(None) };

        // If we successfully parsed the header then there's always at least one
        // entry. We want the last entry.
        let header = headers.iter().last().unwrap();
        ensure_eq_status!(header, "100-continue", 400, "malformed `Expect` header");

        Ok(Some(Self { _priv: () }))
    }
}

impl Header for Expect {
    fn header_name(&self) -> HeaderName {
        EXPECT
    }
    fn header_value(&self) -> HeaderValue {
        let value = "100-continue";
        // SAFETY: the internal string is validated to be ASCII.
        unsafe { HeaderValue::from_bytes_unchecked(value.into()) }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::headers::Headers;

    #[test]
    fn smoke() -> crate::Result<()> {
        let expect = Expect::new();

        let mut headers = Headers::new();
        expect.apply_header(&mut headers);

        let expect = Expect::from_headers(headers)?.unwrap();
        assert_eq!(expect, Expect::new());
        Ok(())
    }

    #[test]
    fn bad_request_on_parse_error() {
        let mut headers = Headers::new();
        headers.insert(EXPECT, "<nori ate the tag. yum.>").unwrap();
        let err = Expect::from_headers(headers).unwrap_err();
        assert_eq!(err.status(), 400);
    }
}
