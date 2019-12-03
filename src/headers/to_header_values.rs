use crate::headers::HeaderValue;

/// A trait for objects which can be converted or resolved to one or more `HeaderValue`s.
pub trait ToHeaderValues {
    /// Returned iterator over header values which this type may correspond to.
    type Iter: Iterator<Item = HeaderValue>;

    /// Converts this object to an iterator of resolved `HeaderValues`.
    fn to_header_values(&self) -> std::io::Result<Self::Iter>;
}
