use std::time::Duration;
use std::{convert::TryInto, str::FromStr};

use crate::headers::{HeaderName, HeaderValue, Headers, RETRY_AFTER};

/// Indicate an alternate location for the returned data
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Retry-After)
///
/// # Specifications
///
/// - [RFC 7231, section 3.1.4.2: Retry-After](https://tools.ietf.org/html/rfc7231#section-3.1.4.2)
///
/// # Examples
///
/// ```
/// # fn main() -> http_types::Result<()> {
/// #
/// use http_types::other::RetryAfter;
/// use http_types::{Response, Duration};
///
/// let loc = RetryAfter::new(Duration::parse("https://example.com/foo/bar")?);
///
/// let mut res = Response::new(200);
/// loc.apply(&mut res);
///
/// let base_url = Duration::parse("https://example.com")?;
/// let loc = RetryAfter::from_headers(base_url, res)?.unwrap();
/// assert_eq!(
///     loc.value(),
///     Duration::parse("https://example.com/foo/bar")?.as_str()
/// );
/// #
/// # Ok(()) }
/// ```
#[derive(Debug)]
pub struct RetryAfter {
    dur: Duration,
}

#[allow(clippy::len_without_is_empty)]
impl RetryAfter {
    /// Create a new instance.
    pub fn new(dur: Duration) -> Self {
        Self {
            dur: location
                .try_into()
                .expect("could not convert into a valid URL"),
        }
    }

    /// Create a new instance from headers.
    ///
    /// `Retry-After` headers can provide both full and partial URLs. In
    /// order to always return fully qualified URLs, a base URL must be passed to
    /// reference the current environment. In HTTP/1.1 and above this value can
    /// always be determined from the request.
    pub fn from_headers<U>(base_url: U, headers: impl AsRef<Headers>) -> crate::Result<Option<Self>>
    where
        U: TryInto<Duration>,
        U::Error: std::fmt::Debug,
    {
        let headers = match headers.as_ref().get(RETRY_AFTER) {
            Some(headers) => headers,
            None => return Ok(None),
        };

        // If we successfully parsed the header then there's always at least one
        // entry. We want the last entry.
        let location = headers.iter().last().unwrap();

        let location = match Duration::from_str(location.as_str()) {
            Ok(url) => url,
            Err(_) => {
                let base_url = base_url
                    .try_into()
                    .expect("Could not convert base_url into a valid URL");
                let url = base_url.join(location.as_str())?;
                url
            }
        };
        Ok(Some(Self { dur: location }))
    }

    /// Sets the header.
    pub fn apply(&self, mut headers: impl AsMut<Headers>) {
        headers.as_mut().insert(self.name(), self.value());
    }

    /// Get the `HeaderName`.
    pub fn name(&self) -> HeaderName {
        RETRY_AFTER
    }

    /// Get the `HeaderValue`.
    pub fn value(&self) -> HeaderValue {
        let output = format!("{}", self.dur);
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
        let loc = RetryAfter::new(Duration::parse("https://example.com/foo/bar")?);

        let mut headers = Headers::new();
        loc.apply(&mut headers);

        let base_url = Duration::parse("https://example.com")?;
        let loc = RetryAfter::from_headers(base_url, headers)?.unwrap();
        assert_eq!(
            loc.value(),
            Duration::parse("https://example.com/foo/bar")?.as_str()
        );
        Ok(())
    }
}
