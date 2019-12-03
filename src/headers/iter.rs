use std::collections::hash_map;
use std::iter::Iterator;

/// Iterator over the headers.
#[derive(Debug)]
pub struct Iter<'a> {
    pub(super) internal: hash_map::Iter<'a, String, String>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a String, &'a String);
    fn next(&mut self) -> Option<Self::Item> {
        self.internal.next()
    }
}
