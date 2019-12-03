//! HTTP headers.

use async_std::io;

use std::iter::IntoIterator;
use std::borrow::Borrow;
use std::collections::HashMap;

mod iter;
mod iter_mut;

pub use iter::Iter;
pub use iter_mut::IterMut;

/// A collection of HTTP Headers.
#[derive(Debug)]
pub struct Headers {
    headers: HashMap<String, String>,
}

impl Headers {
    /// Create a new instance.
    pub fn new() -> Self {
        Self {
            headers: HashMap::new(),
        }
    }

    /// Insert a header into the headers.
    // TODO: enforce this key - values are ASCII only, and return a result
    pub fn insert(
        &mut self,
        name: impl AsRef<str>,
        value: impl AsRef<str>,
    ) -> io::Result<Option<String>> {
        let name = name.as_ref().to_owned();
        let value = value.as_ref().to_owned();
        Ok(self.headers.insert(name, value))
    }

    /// Get a header.
    pub fn get(&self, key: impl Borrow<str>) -> Option<&String> {
        self.headers.get(key.borrow())
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

impl<'a> IntoIterator for &'a Headers {
    type Item = (&'a String, &'a String);
    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Headers {
    type Item = (&'a String, &'a mut String);
    type IntoIter = IterMut<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
