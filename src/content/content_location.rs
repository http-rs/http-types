use crate::headers::{Header, HeaderName, HeaderValue, Headers, CONTENT_LOCATION};
use crate::{bail_status as bail, Status, Url};

use std::convert::TryInto;

/// Indicates an alternate location for the returned data.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Location)
///
/// # Specifications
///
/// - [RFC 7231, section 3.1.4.2: Content-Location](https://tools.ietf.org/html/rfc7231#section-3.1.4.2)
///
/// # Examples
///
/// ```
/// # fn main() -> http_types::Result<()> {
/// #
/// use http_types::{Response, Url};
/// use http_types::content::ContentLocation;
///
/// let content_location = ContentLocation::new(Url::parse("https://example.net/")?);
///
/// let mut res = Response::new(200);
/// res.insert_header(&content_location, &content_location);
///
/// let url = Url::parse("https://example.net/")?;
/// let content_location = ContentLocation::from_headers(url, res)?.unwrap();
/// assert_eq!(content_location.location(), &Url::parse("https://example.net/")?);
/// #
/// # Ok(()) }
/// ```
#[derive(Debug)]
pub struct ContentLocation {
    url: Url,
}

impl ContentLocation {
    /// Create a new instance of `Content-Location` header.
    #[must_use]
    pub fn new(url: Url) -> Self {
        Self { url }
    }

    /// Create a new instance from headers.
    pub fn from_headers<U>(base_url: U, headers: impl AsRef<Headers>) -> crate::Result<Option<Self>>
    where
        U: TryInto<Url>,
        U::Error: std::fmt::Debug,
    {
        let Some(headers) = headers.as_ref().get(CONTENT_LOCATION) else { return Ok(None) };

        // If we successfully parsed the header then there's always at least one
        // entry. We want the last entry.
        let value = headers.iter().last().unwrap();
        let Ok(base) = base_url.try_into() else {bail!(400, "Invalid base url provided")};

        let url = base.join(value.as_str().trim()).status(400)?;
        Ok(Some(Self { url }))
    }

    /// Get the url.
    #[must_use]
    pub fn location(&self) -> &Url {
        &self.url
    }

    /// Set the url.
    pub fn set_location<U>(&mut self, location: U)
    where
        U: TryInto<Url>,
        U::Error: std::fmt::Debug,
    {
        self.url = location
            .try_into()
            .expect("Could not convert into valid URL");
    }
}

impl Header for ContentLocation {
    fn header_name(&self) -> HeaderName {
        CONTENT_LOCATION
    }
    fn header_value(&self) -> HeaderValue {
        let output = self.url.to_string();

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
        let content_location = ContentLocation::new(Url::parse("https://example.net/test.json")?);

        let mut headers = Headers::new();
        content_location.apply_header(&mut headers);

        let content_location =
            ContentLocation::from_headers(Url::parse("https://example.net/").unwrap(), headers)?
                .unwrap();
        assert_eq!(
            content_location.location(),
            &Url::parse("https://example.net/test.json")?
        );
        Ok(())
    }

    #[test]
    fn bad_request_on_parse_error() {
        let mut headers = Headers::new();
        headers
            .insert(CONTENT_LOCATION, "htt://<nori ate the tag. yum.>")
            .unwrap();
        let err =
            ContentLocation::from_headers(Url::parse("https://example.net").unwrap(), headers)
                .unwrap_err();
        assert_eq!(err.status(), 400);
    }
}
