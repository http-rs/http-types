use std::ops::Deref;

use crate::headers::{HeaderName, HeaderValue, Headers};

/// A trait representing a [`HeaderName`] and [`HeaderValue`] pair.
pub trait Header {
    /// The header's name.
    fn header_name(&self) -> HeaderName;

    /// Access the header's value.
    fn header_value(&self) -> HeaderValue;
}

impl Header for (HeaderName, HeaderValue) {
    fn header_name(&self) -> HeaderName {
        self.0
    }

    fn header_value(&self) -> HeaderValue {
        self.1
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
