//! HTTP headers.

use async_std::io;
use std::borrow::Borrow;
use std::collections::HashMap;

mod iter;

pub use iter::Iter;

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

    /// Get an iterator over the headers.
    pub fn iter<'a>(&'a self) -> Iter<'a> {
        Iter {
            internal: self.headers.iter(),
        }
    }
}
