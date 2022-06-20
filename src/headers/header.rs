use std::ops::Deref;

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

        // The value should already have been validated when it was created, so this should
        // possibly be done with an unsafe call
        headers.as_mut().insert(name, value).unwrap();
    }
}

impl Header for (&'static str, &'static str) {
    fn header_name(&self) -> HeaderName {
        if self.0.chars().all(|c| c.is_ascii_lowercase()) {
            HeaderName::from_lowercase_str(self.0)
        } else {
            HeaderName::from(self.0)
        }
    }

    fn header_value(&self) -> HeaderValue {
        HeaderValue::from_static_str(self.1)
    }
}

impl Header for (String, String) {
    fn header_name(&self) -> HeaderName {
        self.0.parse().expect("Header name should be valid ASCII")
    }

    fn header_value(&self) -> HeaderValue {
        self.1.parse().expect("Header value should be valid ASCII")
    }
}

impl<'a, T: Header> Header for &'a T {
    fn header_name(&self) -> HeaderName {
        self.deref().header_name()
    }

    fn header_value(&self) -> HeaderValue {
        self.deref().header_value()
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
