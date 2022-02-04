use crate::errors::HeaderError;
use crate::headers::{Header, HeaderName, HeaderValue, Headers, SOURCE_MAP};
use crate::Url;

use std::convert::TryInto;

/// Links to a file that maps transformed source to the original source.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/SourceMap)
///
/// # Specifications
///
/// - [Source Map Revision 3](https://sourcemaps.info/spec.html)
///
/// # Examples
///
/// ```
/// # fn main() -> anyhow::Result<()> {
/// #
/// use http_types::{Response, Url};
/// use http_types::other::SourceMap;
///
/// let source_map = SourceMap::new(Url::parse("https://example.net/")?);
///
/// let mut res = Response::new(200);
/// res.insert_header(&source_map, &source_map);
///
/// let base_url = Url::parse("https://example.net/")?;
/// let source_map = SourceMap::from_headers(base_url, res)?.unwrap();
/// assert_eq!(source_map.location(), &Url::parse("https://example.net/")?);
/// #
/// # Ok(()) }
/// ```
#[derive(Debug)]
pub struct SourceMap {
    location: Url,
}

impl SourceMap {
    /// Create a new instance of `SourceMap` header.
    pub fn new(location: Url) -> Self {
        Self { location }
    }

    /// Create a new instance from headers.
    pub fn from_headers<U>(base_url: U, headers: impl AsRef<Headers>) -> crate::Result<Option<Self>>
    where
        U: TryInto<Url>,
        U::Error: std::fmt::Debug + Send + Sync + 'static,
    {
        let headers = match headers.as_ref().get(SOURCE_MAP) {
            Some(headers) => headers,
            None => return Ok(None),
        };

        // If we successfully parsed the header then there's always at least one
        // entry. We want the last entry.
        let header_value = headers.iter().last().unwrap();

        let url = match Url::parse(header_value.as_str()) {
            Ok(url) => url,
            Err(_) => match base_url.try_into() {
                Ok(base_url) => base_url
                    .join(header_value.as_str().trim())
                    .map_err(HeaderError::SourceMapInvalidUrl)?,
                Err(e) => return Err(HeaderError::SourceMapInvalidBaseUrl(Box::new(e)).into()),
            },
        };

        Ok(Some(Self { location: url }))
    }

    /// Get the url.
    pub fn location(&self) -> &Url {
        &self.location
    }

    /// Set the url.
    pub fn set_location<U>(&mut self, location: U) -> Result<(), U::Error>
    where
        U: TryInto<Url>,
        U::Error: std::fmt::Debug,
    {
        self.location = location.try_into()?;
        Ok(())
    }
}

impl Header for SourceMap {
    fn header_name(&self) -> HeaderName {
        SOURCE_MAP
    }

    fn header_value(&self) -> HeaderValue {
        let output = self.location.to_string();

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
        let source_map = SourceMap::new(Url::parse("https://example.net/test.json")?);

        let mut headers = Headers::new();
        source_map.apply_header(&mut headers);

        let base_url = Url::parse("https://example.net/")?;
        let source_map = SourceMap::from_headers(base_url, headers)?.unwrap();
        assert_eq!(
            source_map.location(),
            &Url::parse("https://example.net/test.json")?
        );
        Ok(())
    }

    #[test]
    fn bad_request_on_parse_error() {
        let mut headers = Headers::new();
        headers
            .insert(SOURCE_MAP, "htt://<nori ate the tag. yum.>")
            .unwrap();
        let err = SourceMap::from_headers(Url::parse("https://example.net").unwrap(), headers)
            .unwrap_err();
        assert_eq!(err.associated_status_code(), None);
    }

    #[test]
    fn fallback_works() -> anyhow::Result<()> {
        let mut headers = Headers::new();
        headers.insert(SOURCE_MAP, "/test.json").unwrap();

        let base_url = Url::parse("https://fallback.net/")?;
        let source_map = SourceMap::from_headers(base_url, headers)?.unwrap();
        assert_eq!(
            source_map.location(),
            &Url::parse("https://fallback.net/test.json")?
        );

        let mut headers = Headers::new();
        headers
            .insert(SOURCE_MAP, "https://example.com/test.json")
            .unwrap();

        let base_url = Url::parse("https://fallback.net/")?;
        let source_map = SourceMap::from_headers(base_url, headers)?.unwrap();
        assert_eq!(
            source_map.location(),
            &Url::parse("https://example.com/test.json")?
        );
        Ok(())
    }
}
