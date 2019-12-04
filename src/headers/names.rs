use std::collections::hash_map;
use std::iter::Iterator;

use crate::headers::{HeaderName, HeaderValue};

/// Iterator over the headers.
#[derive(Debug)]
pub struct Names<'a> {
    pub(super) internal: hash_map::Keys<'a, HeaderName, Vec<HeaderValue>>,
}

impl<'a> Iterator for Names<'a> {
    type Item = &'a HeaderName;

    fn next(&mut self) -> Option<Self::Item> {
        self.internal.next()
    }
}
