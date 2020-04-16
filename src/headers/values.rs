use std::collections::hash_map;
use std::iter::Iterator;

use crate::headers::{HeaderName, HeaderValue};

/// Iterator over the headers.
#[derive(Debug)]
pub struct Values<'a> {
    pub(super) inner: hash_map::Values<'a, HeaderName, Vec<HeaderValue>>,
    slot: Option<&'a Vec<HeaderValue>>,
    cursor: usize,
}

impl<'a> Values<'a> {
    pub(crate) fn new(inner: hash_map::Values<'a, HeaderName, Vec<HeaderValue>>) -> Self {
        Self {
            inner,
            slot: None,
            cursor: 0,
        }
    }
}

impl<'a> Iterator for Values<'a> {
    type Item = &'a HeaderValue;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Check if we have a vec in the current slot, and if not set one.
            if self.slot.is_none() {
                let next = self.inner.next()?;
                self.cursor = 0;
                self.slot = Some(next);
            }

            // Get the next item
            match self.slot.unwrap().get(self.cursor) {
                // If an item is found, increment the cursor and return the item.
                Some(item) => {
                    self.cursor += 1;
                    return Some(item);
                }
                // If no item is found, unset the slot and loop again.
                None => {
                    self.slot = None;
                    continue;
                }
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
