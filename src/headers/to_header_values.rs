use std::borrow::Cow;
use std::iter;
use std::option;
use std::slice;

use crate::headers::{Header, HeaderValue, HeaderValues, Values};

/// A trait for objects which can be converted or resolved to one or more `HeaderValue`s.
pub trait ToHeaderValues {
    /// Returned iterator over header values which this type may correspond to.
    type Iter: Iterator<Item = HeaderValue>;

    /// Converts this object to an iterator of resolved `HeaderValues`.
    fn to_header_values(&self) -> crate::Result<Self::Iter>;
}

impl<T: Header> ToHeaderValues for T {
    type Iter = option::IntoIter<HeaderValue>;

    fn to_header_values(&self) -> crate::Result<Self::Iter> {
        Ok(Some(self.header_value()).into_iter())
    }
}

impl ToHeaderValues for HeaderValue {
    type Iter = option::IntoIter<HeaderValue>;

    fn to_header_values(&self) -> crate::Result<Self::Iter> {
        Ok(Some(self.clone()).into_iter())
    }
}

impl<'a> ToHeaderValues for &'a HeaderValues {
    type Iter = iter::Cloned<Values<'a>>;

    fn to_header_values(&self) -> crate::Result<Self::Iter> {
        Ok(self.iter().cloned())
    }
}

impl<'a> ToHeaderValues for &'a [HeaderValue] {
    type Iter = iter::Cloned<slice::Iter<'a, HeaderValue>>;

    fn to_header_values(&self) -> crate::Result<Self::Iter> {
        Ok(self.iter().cloned())
    }
}

impl<'a> ToHeaderValues for &'a str {
    type Iter = option::IntoIter<HeaderValue>;

    fn to_header_values(&self) -> crate::Result<Self::Iter> {
        let value = self.parse()?;
        Ok(Some(value).into_iter())
    }
}

impl ToHeaderValues for String {
    type Iter = option::IntoIter<HeaderValue>;

    fn to_header_values(&self) -> crate::Result<Self::Iter> {
        self.as_str().to_header_values()
    }
}

impl ToHeaderValues for &String {
    type Iter = option::IntoIter<HeaderValue>;

    fn to_header_values(&self) -> crate::Result<Self::Iter> {
        self.as_str().to_header_values()
    }
}

impl ToHeaderValues for Cow<'_, str> {
    type Iter = option::IntoIter<HeaderValue>;

    fn to_header_values(&self) -> crate::Result<Self::Iter> {
        self.as_ref().to_header_values()
    }
}
