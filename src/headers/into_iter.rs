use std::collections::hash_map;
use std::iter::Iterator;

/// An owning iterator over the entries of `Headers`.
#[derive(Debug)]
pub struct IntoIter {
    pub(super) internal: hash_map::IntoIter<String, String>,
}

impl Iterator for IntoIter {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        self.internal.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.internal.size_hint()
    }
}
