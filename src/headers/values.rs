use std::collections::hash_map;
use std::iter::Iterator;

use crate::headers::{HeaderName, HeaderValue};

/// Iterator over the headers.
#[derive(Debug)]
pub struct Values<'a> {
    pub(super) internal: hash_map::Values<'a, HeaderName, Vec<HeaderValue>>,
    slot: Option<&'a Vec<HeaderValue>>,
    cursor: usize,
}

impl<'a> Values<'a> {
    pub(crate) fn new(internal: hash_map::Values<'a, HeaderName, Vec<HeaderValue>>) -> Self {
        Self {
            internal,
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
            if let None = self.slot {
                let next = self.internal.next();
                if next.is_none() {
                    return None;
                }
                self.cursor = 0;
                self.slot = next;
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
}
