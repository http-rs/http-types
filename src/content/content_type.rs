use std::{convert::TryInto, str::FromStr};

use crate::errors::HeaderError;
use crate::headers::{Header, HeaderName, HeaderValue, Headers, CONTENT_TYPE};
use crate::mime::Mime;

/// Indicate the media type of a resource's content.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Type)
///
/// # Specifications
///
/// - [RFC 7231, section 3.1.1.5: Content-Type](https://tools.ietf.org/html/rfc7231#section-3.1.1.5)
/// - [RFC 7233, section 4.1: Content-Type in multipart](https://tools.ietf.org/html/rfc7233#section-4.1)
///
/// # Examples
///
/// ```
/// # fn main() -> anyhow::Result<()> {
/// #
/// use http_types::content::ContentType;
/// use http_types::{headers::Header, Response};
/// use http_types::mime::Mime;
/// use std::str::FromStr;
///
/// let content_type = ContentType::new("text/*");
///
/// let mut res = Response::new(200);
/// res.insert_header(&content_type, &content_type);
///
/// let content_type = ContentType::from_headers(res)?.unwrap();
/// assert_eq!(content_type.header_value(), format!("{}", Mime::from_str("text/*")?).as_str());
/// #
/// # Ok(()) }
/// ```
#[derive(Debug)]
pub struct ContentType {
    media_type: Mime,
}

impl ContentType {
    /// Create a new instance.
    pub fn new<U>(media_type: U) -> Self
    where
        U: TryInto<Mime>,
        U::Error: std::fmt::Debug,
    {
        Self {
            media_type: media_type
                .try_into()
                .expect("could not convert into a valid Mime type"),
        }
    }

    /// Create a new instance from headers.
    ///
    /// `Content-Type` headers can provide both full and partial URLs. In
    /// order to always return fully qualified URLs, a base URL must be passed to
    /// reference the current environment. In HTTP/1.1 and above this value can
    /// always be determined from the request.
    pub fn from_headers(headers: impl AsRef<Headers>) -> crate::Result<Option<Self>> {
        let headers = match headers.as_ref().get(CONTENT_TYPE) {
            Some(headers) => headers,
            None => return Ok(None),
        };

        // If we successfully parsed the header then there's always at least one
        // entry. We want the last entry.
        let ctation = headers.iter().last().unwrap();

        let media_type =
            Mime::from_str(ctation.as_str()).map_err(HeaderError::ContentTypeInvalidMediaType)?;
        Ok(Some(Self { media_type }))
    }
}

impl Header for ContentType {
    fn header_name(&self) -> HeaderName {
        CONTENT_TYPE
    }
    fn header_value(&self) -> HeaderValue {
        let output = format!("{}", self.media_type);
        // SAFETY: the internal string is validated to be ASCII.
        unsafe { HeaderValue::from_bytes_unchecked(output.into()) }
    }
}

impl PartialEq<Mime> for ContentType {
    fn eq(&self, other: &Mime) -> bool {
        &self.media_type == other
    }
}

impl PartialEq<&Mime> for ContentType {
    fn eq(&self, other: &&Mime) -> bool {
        &&self.media_type == other
    }
}

impl From<Mime> for ContentType {
    fn from(media_type: Mime) -> Self {
        Self { media_type }
    }
}

#[cfg(test)]
mod test {
    use crate::headers::Headers;
    use crate::StatusCode;

    use super::*;

    #[test]
    fn smoke() -> anyhow::Result<()> {
        let ct = ContentType::new(Mime::from_str("text/*")?);

        let mut headers = Headers::new();
        ct.apply_header(&mut headers);

        let ct = ContentType::from_headers(headers)?.unwrap();
        assert_eq!(
            ct.header_value(),
            format!("{}", Mime::from_str("text/*")?).as_str()
        );
        Ok(())
    }

    #[test]
    fn bad_request_on_parse_error() {
        let mut headers = Headers::new();
        headers
            .insert(CONTENT_TYPE, "<nori ate the tag. yum.>")
            .unwrap();
        let err = ContentType::from_headers(headers).unwrap_err();
        assert_eq!(err.associated_status_code(), Some(StatusCode::BadRequest));
    }
}
