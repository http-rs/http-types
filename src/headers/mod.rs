//! HTTP headers.

use async_std::io;

use std::borrow::Borrow;
use std::collections::HashMap;
use std::iter::IntoIterator;

mod header_name;
mod header_value;
mod into_iter;
mod iter;
mod iter_mut;
mod to_header_values;

pub use header_name::HeaderName;
pub use header_value::HeaderValue;
pub use into_iter::IntoIter;
pub use iter::Iter;
pub use iter_mut::IterMut;
pub use to_header_values::ToHeaderValues;

/// A collection of HTTP Headers.
#[derive(Debug)]
pub struct Headers {
    headers: HashMap<HeaderName, HeaderValue>,
}

impl Headers {
    /// Create a new instance.
    pub(crate) fn new() -> Self {
        Self {
            headers: HashMap::new(),
        }
    }

    /// Insert a header into the headers.
    pub fn insert(
        &mut self,
        name: HeaderName,
        value: HeaderValue,
    ) -> io::Result<Option<HeaderValue>> {
        Ok(self.headers.insert(name, value))
    }

    /// Get a header.
    pub fn get(&self, name: impl Borrow<HeaderName>) -> Option<&HeaderValue> {
        self.headers.get(name.borrow())
    }

    /// An iterator visiting all header pairs in arbitrary order.
    pub fn iter<'a>(&'a self) -> Iter<'a> {
        Iter {
            internal: self.headers.iter(),
        }
    }

    /// An iterator visiting all header pairs in arbitrary order, with mutable references to the
    /// values.
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a> {
        IterMut {
            internal: self.headers.iter_mut(),
        }
    }
}

impl IntoIterator for Headers {
    type Item = (HeaderName, HeaderValue);
    type IntoIter = IntoIter;

    /// Returns a iterator of references over the remaining items.
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            internal: self.headers.into_iter(),
        }
    }
}

impl<'a> IntoIterator for &'a Headers {
    type Item = (&'a HeaderName, &'a HeaderValue);
    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Headers {
    type Item = (&'a HeaderName, &'a mut HeaderValue);
    type IntoIter = IterMut<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
