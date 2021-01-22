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

impl<'a, 'b> Header for (&'a str, &'b str) {
    fn header_name(&self) -> HeaderName {
        HeaderName::from(self.0)
    }

    fn header_value(&self) -> HeaderValue {
        HeaderValue::from_bytes(self.1.to_owned().into_bytes())
            .expect("String slice should be valid ASCII")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn header_from_strings() {
        let strings = ("Content-Length", "12");
        assert_eq!(strings.header_name(), "Content-Length");
        assert_eq!(strings.header_value(), "12");
    }
}
