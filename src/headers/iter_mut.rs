use std::collections::hash_map;
use std::iter::Iterator;

use crate::headers::{HeaderName, HeaderValue};

/// Iterator over the headers.
#[derive(Debug)]
pub struct IterMut<'a> {
    pub(super) inner: hash_map::IterMut<'a, HeaderName, Vec<HeaderValue>>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = (&'a HeaderName, &'a mut Vec<HeaderValue>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
