//! HTTP headers.

use std::collections::HashMap;
// use std::iter::{IntoIterator, Iterator};

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
}
