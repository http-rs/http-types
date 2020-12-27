use crate::headers::{HeaderName, HeaderValue, Headers};

/// A trait representing a [`HeaderName`] and [`HeaderValue`] pair.
pub trait Header {
    /// Access the header's name.
    fn header_name(&self) -> HeaderName;

    /// Access the header's value.
    fn header_value(&self) -> HeaderValue;

    /// Insert the header name and header value into something that looks like a
    /// [`Headers`] map.
    fn apply_header<H: AsMut<Headers>>(&self, mut headers: H) {
        let name = self.header_name();
        let value = self.header_value();
        headers.as_mut().insert(name, value);
    }
}
