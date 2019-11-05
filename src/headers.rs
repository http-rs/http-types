//! HTTP headers.

use std::collections::HashMap;
// use std::iter::{IntoIterator, Iterator};

/// A collection of HTTP Headers.
#[derive(Debug)]
pub struct Headers<'a> {
    headers: HashMap<String, String>,
    __marker: &'a std::marker::PhantomData<()>,
}
