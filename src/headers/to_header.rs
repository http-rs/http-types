use std::convert::TryInto;

use crate::headers::{HeaderName, HeaderValue};

/// A trait for objects which can be converted or resolved to a `HeaderName` and `HeaderValue` pair.
pub trait ToHeader {
    /// Converts this object to a `HeaderName` and `HeaderValue` pair.
    fn to_header(self) -> crate::Result<(HeaderName, HeaderValue)>;
}

impl<N, V> ToHeader for (N, V)
where
    N: TryInto<HeaderName>,
    V: TryInto<HeaderValue>,
    <N as TryInto<HeaderName>>::Error: Into<crate::Error>,
    <V as TryInto<HeaderValue>>::Error: Into<crate::Error>,
{
    fn to_header(self) -> crate::Result<(HeaderName, HeaderValue)> {
        Ok((
            self.0.try_into().map_err(Into::into)?,
            self.1.try_into().map_err(Into::into)?,
        ))
    }
}
