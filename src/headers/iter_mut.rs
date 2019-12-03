use std::collections::hash_map;
use std::iter::Iterator;

/// Iterator over the headers.
#[derive(Debug)]
pub struct IterMut<'a> {
    pub(super) internal: hash_map::IterMut<'a, String, String>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = (&'a String, &'a mut String);

    fn next(&mut self) -> Option<Self::Item> {
        self.internal.next()
    }
}
