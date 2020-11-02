use crate::bail_status;
use crate::headers::{HeaderName, HeaderValue, Headers, ToHeaderValues, SAVE_DATA};

use std::fmt::Debug;
use std::option;

/// HTTP `SaveData` header
///
/// This header is considered "experimental" and may be subject to change as the
/// spec evolves.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Save-Data)
///
/// # Specifications
///
/// - [draft-grigorik-http-client-hints-03, section 7: Save-Data](https://tools.ietf.org/html/draft-grigorik-http-client-hints-03#section-7)
///
/// # Examples
///
/// ```
/// # fn main() -> http_types::Result<()> {
/// #
/// use http_types::Response;
/// use http_types::client_hints::SaveData;
///
/// let expect = SaveData::new(true);
///
/// let mut res = Response::new(200);
/// expect.apply(&mut res);
///
/// let expect = SaveData::from_headers(res)?.unwrap();
/// assert_eq!(expect, SaveData::new(true));
/// #
/// # Ok(()) }
/// ```
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct SaveData {
    enabled: bool,
}

impl SaveData {
    /// Create a new instance of `SaveData`.
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    /// Create an instance of `SaveData` from a `Headers` instance.
    pub fn from_headers(headers: impl AsRef<Headers>) -> crate::Result<Option<Self>> {
        let headers = match headers.as_ref().get(SAVE_DATA) {
            Some(headers) => headers,
            None => return Ok(None),
        };

        // If we successfully parsed the header then there's always at least one
        // entry. We want the last entry.
        let header = headers.iter().last().unwrap();
        let enabled = match header.as_str() {
            "1" => true,
            "0" => false,
            _ => bail_status!(400, "malformed `SaveData` header"),
        };

        Ok(Some(Self { enabled }))
    }

    /// Insert a `HeaderName` + `HeaderValue` pair into a `Headers` instance.
    pub fn apply(&self, mut headers: impl AsMut<Headers>) {
        headers.as_mut().insert(SAVE_DATA, self.value());
    }

    /// Get the `HeaderName`.
    pub fn name(&self) -> HeaderName {
        SAVE_DATA
    }

    /// Get the `HeaderValue`.
    pub fn value(&self) -> HeaderValue {
        let value = if self.enabled { "1" } else { "0" };
        // SAFETY: the internal string is validated to be ASCII.
        unsafe { HeaderValue::from_bytes_unchecked(value.into()) }
    }
}

impl ToHeaderValues for SaveData {
    type Iter = option::IntoIter<HeaderValue>;
    fn to_header_values(&self) -> crate::Result<Self::Iter> {
        // A HeaderValue will always convert into itself.
        Ok(self.value().to_header_values().unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::headers::Headers;

    #[test]
    fn smoke() -> crate::Result<()> {
        let expect = SaveData::new(true);

        let mut headers = Headers::new();
        expect.apply(&mut headers);

        let expect = SaveData::from_headers(headers)?.unwrap();
        assert_eq!(expect, SaveData::new(true));
        Ok(())
    }

    #[test]
    fn bad_request_on_parse_error() -> crate::Result<()> {
        let mut headers = Headers::new();
        headers.insert(SAVE_DATA, "<nori ate the tag. yum.>");
        let err = SaveData::from_headers(headers).unwrap_err();
        assert_eq!(err.status(), 400);
        Ok(())
    }
}
