use crate::headers::HeaderValue;
use std::io;
use std::option;

/// A trait for objects which can be converted or resolved to one or more `HeaderValue`s.
pub trait ToHeaderValues {
    /// Returned iterator over header values which this type may correspond to.
    type Iter: Iterator<Item = HeaderValue>;

    /// Converts this object to an iterator of resolved `HeaderValues`.
    fn to_header_values(&self) -> io::Result<Self::Iter>;
}

impl ToHeaderValues for HeaderValue {
    type Iter = option::IntoIter<HeaderValue>;

    fn to_header_values(&self) -> io::Result<Self::Iter> {
        Ok(Some(self.clone()).into_iter())
    }
}
