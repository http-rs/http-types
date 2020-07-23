//! Metrics and descriptions for the given request-response cycle.
//!
//! # Examples
//!
//! ```
//! # fn main() -> http_types::Result<()> {
//! #
//! use http_types::Response;
//! use http_types::trace::{ServerTiming, Metric};
//!
//! let mut timings = ServerTiming::new();
//! timings.push(Metric::new("server".to_owned(), None, None)?);
//!
//! let mut res = Response::new(200);
//! timings.apply(&mut res);
//!
//! let timings = ServerTiming::from_headers(res)?;
//! let entry = timings.iter().next().unwrap();
//! assert_eq!(entry.name(), "server");
//! #
//! # Ok(()) }
//! ```

mod metric;
mod parse;

pub use metric::Metric;
use parse::parse_header;

use std::convert::AsMut;
use std::iter::Iterator;
use std::slice;

use crate::headers::HeaderValue;
use crate::Headers;

/// Metrics and descriptions for the given request-response cycle.
///
/// This is an implementation of the W3C [Server
/// Timing](https://w3c.github.io/server-timing/#the-server-timing-header-field)
/// header spec. Read more on
/// [MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Server-Timing).
///
/// # Examples
///
/// ```
/// # fn main() -> http_types::Result<()> {
/// #
/// use http_types::Response;
/// use http_types::trace::{ServerTiming, Metric};
///
/// let mut timings = ServerTiming::new();
/// timings.push(Metric::new("server".to_owned(), None, None)?);
///
/// let mut res = Response::new(200);
/// timings.apply(&mut res);
///
/// let timings = ServerTiming::from_headers(res)?;
/// let entry = timings.iter().next().unwrap();
/// assert_eq!(entry.name(), "server");
/// #
/// # Ok(()) }
/// ```
#[derive(Debug)]
pub struct ServerTiming {
    timings: Vec<Metric>,
}

impl ServerTiming {
    /// Create a new instance of `ServerTiming`.
    pub fn new() -> Self {
        Self { timings: vec![] }
    }
    /// Create a new instance from headers.
    pub fn from_headers(headers: impl AsRef<Headers>) -> crate::Result<Self> {
        let mut timings = vec![];
        let values = headers.as_ref().get("server-timing");
        for value in values.iter().map(|h| h.iter()).flatten() {
            parse_header(value.as_str(), &mut timings)?;
        }
        Ok(Self { timings })
    }

    /// Sets the `Server-Timing` header.
    pub fn apply(&self, mut headers: impl AsMut<Headers>) {
        for timing in &self.timings {
            let value: HeaderValue = timing.clone().into();
            headers.as_mut().insert("server-timing", value);
        }
    }

    /// Push an entry into the list of entries.
    pub fn push(&mut self, entry: Metric) {
        self.timings.push(entry);
    }

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
    type Item = Metric;
    type IntoIter = IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<'a> IntoIterator for &'a ServerTiming {
    type Item = &'a Metric;
    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut ServerTiming {
    type Item = &'a mut Metric;
    type IntoIter = IterMut<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/// A borrowing iterator over entries in `ServerTiming`.
#[derive(Debug)]
pub struct IntoIter {
    inner: std::vec::IntoIter<Metric>,
}

impl Iterator for IntoIter {
    type Item = Metric;

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
    inner: slice::Iter<'a, Metric>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Metric;

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
    inner: slice::IterMut<'a, Metric>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut Metric;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::headers::Headers;

    #[test]
    fn smoke() -> crate::Result<()> {
        let mut timings = ServerTiming::new();
        timings.push(Metric::new("server".to_owned(), None, None)?);

        let mut headers = Headers::new();
        timings.apply(&mut headers);

        let timings = ServerTiming::from_headers(headers)?;
        let entry = timings.iter().next().unwrap();
        assert_eq!(entry.name(), "server");
        Ok(())
    }
}
