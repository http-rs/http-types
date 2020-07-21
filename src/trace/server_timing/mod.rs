//! Metrics and descriptions for the given request-response cycle.

mod entry;

pub use entry::Entry;

use std::iter::Iterator;
use std::slice;

/// Metrics and descriptions for the given request-response cycle.
///
/// This is an implementation of the W3C [Server
/// Timing](https://w3c.github.io/server-timing/#the-server-timing-header-field)
/// header spec. Read more on
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Server-Timing).
#[derive(Debug)]
pub struct ServerTiming {
    timings: Vec<Entry>,
}

impl ServerTiming {
    /// An iterator visiting all server timings.
    pub fn into_iter(self) -> IntoIter {
        IntoIter {
            inner: self.timings.into_iter(),
        }
    }

    /// An iterator visiting all server timings.
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            inner: self.timings.iter(),
        }
    }

    /// An iterator visiting all server timings.
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut {
            inner: self.timings.iter_mut(),
        }
    }
}

impl IntoIterator for ServerTiming {
    type Item = Entry;
    type IntoIter = IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<'a> IntoIterator for &'a ServerTiming {
    type Item = &'a Entry;
    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut ServerTiming {
    type Item = &'a mut Entry;
    type IntoIter = IterMut<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/// A borrowing iterator over entries in `ServerTiming`.
#[derive(Debug)]
pub struct IntoIter {
    inner: std::vec::IntoIter<Entry>,
}

impl Iterator for IntoIter {
    type Item = Entry;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

/// A lending iterator over entries in `ServerTiming`.
#[derive(Debug)]
pub struct Iter<'a> {
    inner: slice::Iter<'a, Entry>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Entry;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

/// A mutable iterator over entries in `ServerTiming`.
#[derive(Debug)]
pub struct IterMut<'a> {
    inner: slice::IterMut<'a, Entry>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut Entry;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

mod test {
    const CASE1: &str =
        "Server-Timing: metric1; dur=1.1; desc=document, metric1; dur=1.2; desc=document";
}
