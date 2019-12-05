use std::collections::hash_map;
use std::iter::Iterator;

use crate::headers::{HeaderName, HeaderValue};

/// Iterator over the headers.
#[derive(Debug)]
pub struct ValuesMut<'a> {
    pub(super) inner: hash_map::ValuesMut<'a, HeaderName, Vec<HeaderValue>>,
    slot: Option<&'a mut Vec<HeaderValue>>,
    cursor: usize,
}

impl<'a> ValuesMut<'a> {
    pub(crate) fn new(inner: hash_map::ValuesMut<'a, HeaderName, Vec<HeaderValue>>) -> Self {
        Self {
            inner,
            slot: None,
            cursor: 0,
        }
    }
}

impl<'a> Iterator for ValuesMut<'a> {
    type Item = &'a mut HeaderValue;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Check if we have a vec in the current slot, and if not set one.
            if let None = self.slot {
                let next = self.inner.next();
                if next.is_none() {
                    return None;
                }
                self.cursor = 0;
                self.slot = next;
            }

            // Get the next item
            match self.slot.as_mut().unwrap().get_mut(self.cursor) {
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
