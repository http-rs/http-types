use std::ops::Deref;

use crate::headers::{HeaderName, HeaderValue};

/// A trait representing a [`HeaderName`] and [`HeaderValue`] pair.
pub trait Header
where
    Self: Sized,
{
    /// The header's name.
    fn header_name(&self) -> HeaderName;

    /// Access the header's value.
    fn header_value(&self) -> HeaderValue;

    /// Create an instance from a header value.
    fn from_parts(name: HeaderName, value: HeaderValue) -> crate::Result<Self>;
}

impl Header for (HeaderName, HeaderValue) {
    fn header_name(&self) -> HeaderName {
        self.0
    }

    fn header_value(&self) -> HeaderValue {
        self.1
    }

    fn from_parts(name: HeaderName, value: HeaderValue) -> crate::Result<Self> {
        Ok((name, value))
    }
}

impl<'a, T: Header> Header for &'a T {
    fn header_name(&self) -> HeaderName {
        self.deref().header_name()
    }

    fn header_value(&self) -> HeaderValue {
        self.deref().header_value()
    }

    fn from_parts(name: HeaderName, value: HeaderValue) -> crate::Result<Self> {
        T::from_parts(name, value)
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
