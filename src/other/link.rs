use crate::headers::{HeaderName, HeaderValue, Headers, LINK};
use crate::{bail_status as bail, Status, Url};

use std::convert::TryInto;

use super::LinkDirective;

/// Contains the address of the page making the request.
///
/// __Important__: Although this header has many innocent uses it can have
/// undesirable consequences for user security and privacy.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Referer)
///
/// # Specifications
///
/// - [RFC 8288, section 3: Link Serialisation in HTTP Headers](https://tools.ietf.org/html/rfc8288#section-3)
/// - [RFC 5988, section 5: The Link Header Field](https://tools.ietf.org/html/rfc5988#section-5)
///
/// # Examples
///
/// ```
/// # fn main() -> http_types::Result<()> {
/// #
/// use http_types::{Response, Url};
/// use http_types::other::Referer;
///
/// let referer = Referer::new(Url::parse("https://example.net/")?);
///
/// let mut res = Response::new(200);
/// referer.apply(&mut res);
///
/// let base_url = Url::parse("https://example.net/")?;
/// let referer = Referer::from_headers(base_url, res)?.unwrap();
/// assert_eq!(referer.location(), &Url::parse("https://example.net/")?);
/// #
/// # Ok(()) }
/// ```
#[derive(Debug)]
pub struct Link {
    links: Vec<LinkDirective>,
}

impl Link {
    /// Create a new instance of `Referer` header.
    pub fn new() -> Self {
        Self { links: vec![] }
    }

    /// Create a new instance from headers.
    pub fn from_headers<U>(base_url: U, headers: impl AsRef<Headers>) -> crate::Result<Option<Self>>
    where
        U: TryInto<Url>,
        U::Error: std::fmt::Debug,
    {
        let headers = match headers.as_ref().get(LINK) {
            Some(headers) => headers,
            None => return Ok(None),
        };
        todo!();
    }

    /// Sets the header.
    pub fn apply(&self, mut headers: impl AsMut<Headers>) {
        headers.as_mut().insert(self.name(), self.value());
    }

    /// Get the `HeaderName`.
    pub fn name(&self) -> HeaderName {
        LINK
    }

    /// Get the `HeaderValue`.
    pub fn value(&self) -> HeaderValue {
        todo!();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::headers::Headers;

    #[test]
    fn smoke() -> crate::Result<()> {
        // let referer = Link::new(Url::parse("https://example.net/test.json")?);

        // let mut headers = Headers::new();
        // referer.apply(&mut headers);

        // let base_url = Url::parse("https://example.net/")?;
        // let referer = Link::from_headers(base_url, headers)?.unwrap();
        // assert_eq!(
        //     referer.location(),
        //     &Url::parse("https://example.net/test.json")?
        // );
        Ok(())
    }
}
