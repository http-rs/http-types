use std::{convert::TryInto, str::FromStr};

use crate::headers::{HeaderName, HeaderValue, Headers, CONTENT_LOCATION};
use crate::url::Url;

/// Indicate an alternate location for the returned data
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
/// use http_types::content::ContentLocation;
/// use http_types::{Response, Url};
///
/// let loc = ContentLocation::new(Url::parse("https://example.com/foo/bar")?);
///
/// let mut res = Response::new(200);
/// loc.apply(&mut res);
///
/// let base_url = Url::parse("https://example.com")?;
/// let loc = ContentLocation::from_headers(base_url, res)?.unwrap();
/// assert_eq!(
///     loc.value(),
///     Url::parse("https://example.com/foo/bar")?.as_str()
/// );
/// #
/// # Ok(()) }
/// ```
#[derive(Debug)]
pub struct ContentLocation {
    location: Url,
}

#[allow(clippy::len_without_is_empty)]
impl ContentLocation {
    /// Create a new instance.
    pub fn new<U>(location: U) -> Self
    where
        U: TryInto<Url>,
        U::Error: std::fmt::Debug,
    {
        Self {
            location: location
                .try_into()
                .expect("could not convert into a valid URL"),
        }
    }

    /// Create a new instance from headers.
    ///
    /// `Content-Location` headers can provide both full and partial URLs. In
    /// order to always return fully qualified URLs, a base URL must be passed to
    /// reference the current environment. In HTTP/1.1 and above this value can
    /// always be determined from the request.
    pub fn from_headers<U>(base_url: U, headers: impl AsRef<Headers>) -> crate::Result<Option<Self>>
    where
        U: TryInto<Url>,
        U::Error: std::fmt::Debug,
    {
        let headers = match headers.as_ref().get(CONTENT_LOCATION) {
            Some(headers) => headers,
            None => return Ok(None),
        };

        // If we successfully parsed the header then there's always at least one
        // entry. We want the last entry.
        let location = headers.iter().last().unwrap();

        let location = match Url::from_str(location.as_str()) {
            Ok(url) => url,
            Err(_) => {
                let base_url = base_url
                    .try_into()
                    .expect("Could not convert base_url into a valid URL");
                let url = base_url.join(location.as_str())?;
                url
            }
        };
        Ok(Some(Self { location }))
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
        let output = format!("{}", self.location);
        // SAFETY: the internal string is validated to be ASCII.
        unsafe { HeaderValue::from_bytes_unchecked(output.into()) }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::headers::Headers;

    // NOTE(yosh): I couldn't get a 400 test in because I couldn't generate any
    // invalid URLs. By default they get escaped, so ehhh -- I think it's fine.

    #[test]
    fn smoke() -> crate::Result<()> {
        let loc = ContentLocation::new(Url::parse("https://example.com/foo/bar")?);

        let mut headers = Headers::new();
        loc.apply(&mut headers);

        let base_url = Url::parse("https://example.com")?;
        let loc = ContentLocation::from_headers(base_url, headers)?.unwrap();
        assert_eq!(
            loc.value(),
            Url::parse("https://example.com/foo/bar")?.as_str()
        );
        Ok(())
    }
}
