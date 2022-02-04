use crate::errors::HeaderError;
use crate::headers::{Header, HeaderName, HeaderValue, Headers, CONTENT_LOCATION};
use crate::Url;

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
/// # fn main() -> anyhow::Result<()> {
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
    pub fn new(url: Url) -> Self {
        Self { url }
    }

    /// Create a new instance from headers.
    pub fn from_headers<U>(base_url: U, headers: impl AsRef<Headers>) -> crate::Result<Option<Self>>
    where
        U: TryInto<Url>,
        U::Error: std::fmt::Debug + Send + Sync + 'static,
    {
        let headers = match headers.as_ref().get(CONTENT_LOCATION) {
            Some(headers) => headers,
            None => return Ok(None),
        };

        // If we successfully parsed the header then there's always at least one
        // entry. We want the last entry.
        let header_value = headers.iter().last().unwrap();
        let url = match base_url.try_into() {
            Ok(base_url) => base_url
                .join(header_value.as_str().trim())
                .map_err(HeaderError::ContentLocationInvalidUrl)?,
            Err(e) => return Err(HeaderError::ContentLocationInvalidBaseUrl(Box::new(e)).into()),
        };
        Ok(Some(Self { url }))
    }

    /// Get the url.
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
            .expect("Could not convert into valid URL")
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
    use crate::headers::Headers;

    use super::*;

    #[test]
    fn smoke() -> anyhow::Result<()> {
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
        assert_eq!(err.associated_status_code(), None);
    }
}
