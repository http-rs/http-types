use std::collections::hash_map;
use std::iter::Iterator;

use crate::headers::{HeaderName, HeaderValue};

/// Iterator over the headers.
#[derive(Debug)]
pub struct IterMut<'a> {
    pub(super) internal: hash_map::IterMut<'a, HeaderName, HeaderValue>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = (&'a HeaderName, &'a mut HeaderValue);

    fn next(&mut self) -> Option<Self::Item> {
        self.internal.next()
    }
}
