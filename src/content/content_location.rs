use crate::headers::{HeaderName, HeaderValue, Headers, CONTENT_LOCATION};
use crate::{Status, Url};

///
/// # Specifications
///
/// - [RFC 7230, section 3.3.2: Content-Length](https://tools.ietf.org/html/rfc7230#section-3.3.2)
///
/// # Examples
///
/// ```
/// # fn main() -> http_types::Result<()> {
/// #
/// use http_types::Response;
/// use http_types::content::{ContentLocation};
///
/// let content_location = ContentLocation::new("https://example.net/".to_string());
///
/// let mut res = Response::new(200);
/// content_location.apply(&mut res);
///
/// let content_location = ContentLocation::from_headers(res)?.unwrap();
/// assert_eq!(content_location.location(), "https://example.net/");
/// #
/// # Ok(()) }
/// ```
#[derive(Debug)]
pub struct ContentLocation {
    url: String,
}

#[allow(clippy::len_without_is_empty)]
impl ContentLocation {
    /// Create a new instance of `Content-Location` header.
    pub fn new(url: String) -> Self {
        Self { url }
    }

    /// Create a new instance from headers.
    pub fn from_headers(headers: impl AsRef<Headers>) -> crate::Result<Option<Self>> {
        let headers = match headers.as_ref().get(CONTENT_LOCATION) {
            Some(headers) => headers,
            None => return Ok(None),
        };

        // If we successfully parsed the header then there's always at least one
        // entry. We want the last entry.
        let value = headers.iter().last().unwrap();
        let url = Url::parse(value.as_str().trim()).status(400)?;
        Ok(Some(Self { url : url.into_string() }))
    }

    /// Sets the header.
    pub fn apply(&self, mut headers: impl AsMut<Headers>) {
        headers.as_mut().insert(self.name(), self.value());
    }

    /// Get the `HeaderName`.
    pub fn name(&self) -> HeaderName {
        CONTENT_LOCATION
    }

    /// Get the `HeaderValue`.
    pub fn value(&self) -> HeaderValue {
        let output = format!("{}", self.url);

        // SAFETY: the internal string is validated to be ASCII.
        unsafe { HeaderValue::from_bytes_unchecked(output.into()) }
    }

    /// Get the url.
    pub fn location(&self) -> String {
        String::from(&self.url)
    }

    /// Set the url.
    pub fn set_location(&mut self, location: &str) {
        self.url = location.to_string();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::headers::Headers;

    #[test]
    fn smoke() -> crate::Result<()> {
        let content_location = ContentLocation::new("https://example.net/test".to_string());

        let mut headers = Headers::new();
        content_location.apply(&mut headers);

        let content_location = ContentLocation::from_headers(headers)?.unwrap();
        assert_eq!(content_location.location(), "https://example.net/test");
        Ok(())
    }

    #[test]
    fn bad_request_on_parse_error() -> crate::Result<()> {
        let mut headers = Headers::new();
        headers.insert(CONTENT_LOCATION, "<nori ate the tag. yum.>");
        let err = ContentLocation::from_headers(headers).unwrap_err();
        assert_eq!(err.status(), 400);
        Ok(())
    }
}
