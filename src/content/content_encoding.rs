//! Specify the compression algorithm.

use crate::content::Encoding;
use crate::headers::{HeaderName, HeaderValue, Headers, ToHeaderValues, CONTENT_ENCODING};

use std::fmt::{self, Debug, Write};
use std::iter::Iterator;
use std::option;
use std::slice;

/// Specify the compression algorithm.
///
/// # Specifications
///
/// - [RFC 7231, section 3.1.2.2: Content-Encoding](https://tools.ietf.org/html/rfc7231#section-3.1.2.2)
///
/// # Examples
///
/// ```
/// # fn main() -> http_types::Result<()> {
/// #
/// use http_types::Response;
/// use http_types::content::{ContentEncoding, Encoding};
/// let mut entries = ContentEncoding::new();
/// entries.push(Encoding::Gzip);
/// entries.push(Encoding::Identity);
///
/// let mut res = Response::new(200);
/// entries.apply(&mut res);
///
/// let entries = ContentEncoding::from_headers(res)?.unwrap();
/// let mut entries = entries.iter();
/// assert_eq!(entries.next().unwrap(), &Encoding::Gzip);
/// assert_eq!(entries.next().unwrap(), &Encoding::Identity);
/// #
/// # Ok(()) }
/// ```
pub struct ContentEncoding {
    entries: Vec<Encoding>,
}

impl ContentEncoding {
    /// Create a new instance of `CacheControl`.
    pub fn new() -> Self {
        Self { entries: vec![] }
    }

    /// Create a new instance from headers.
    pub fn from_headers(headers: impl AsRef<Headers>) -> crate::Result<Option<Self>> {
        let mut entries = vec![];
        let headers = match headers.as_ref().get(CONTENT_ENCODING) {
            Some(headers) => headers,
            None => return Ok(None),
        };

        for value in headers {
            for part in value.as_str().trim().split(',') {
                // Try and parse a directive from a str. If the directive is
                // unkown we skip it.
                if let Some(entry) = Encoding::from_str(part) {
                    entries.push(entry);
                }
            }
        }

        Ok(Some(Self { entries }))
    }

    /// Sets the `Server-Timing` header.
    pub fn apply(&self, mut headers: impl AsMut<Headers>) {
        headers.as_mut().insert(CONTENT_ENCODING, self.value());
    }

    /// Get the `HeaderName`.
    pub fn name(&self) -> HeaderName {
        CONTENT_ENCODING
    }

    /// Get the `HeaderValue`.
    pub fn value(&self) -> HeaderValue {
        let mut output = String::new();
        for (n, directive) in self.entries.iter().enumerate() {
            let directive: HeaderValue = directive.clone().into();
            match n {
                0 => write!(output, "{}", directive).unwrap(),
                _ => write!(output, ", {}", directive).unwrap(),
            };
        }

        // SAFETY: the internal string is validated to be ASCII.
        unsafe { HeaderValue::from_bytes_unchecked(output.into()) }
    }
    /// Push a directive into the list of entries.
    pub fn push(&mut self, directive: Encoding) {
        self.entries.push(directive);
    }

    /// An iterator visiting all server entries.
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            inner: self.entries.iter(),
        }
    }

    /// An iterator visiting all server entries.
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut {
            inner: self.entries.iter_mut(),
        }
    }
}

impl IntoIterator for ContentEncoding {
    type Item = Encoding;
    type IntoIter = IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.entries.into_iter(),
        }
    }
}

impl<'a> IntoIterator for &'a ContentEncoding {
    type Item = &'a Encoding;
    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut ContentEncoding {
    type Item = &'a mut Encoding;
    type IntoIter = IterMut<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/// A borrowing iterator over entries in `CacheControl`.
#[derive(Debug)]
pub struct IntoIter {
    inner: std::vec::IntoIter<Encoding>,
}

impl Iterator for IntoIter {
    type Item = Encoding;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

/// A lending iterator over entries in `CacheControl`.
#[derive(Debug)]
pub struct Iter<'a> {
    inner: slice::Iter<'a, Encoding>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Encoding;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

/// A mutable iterator over entries in `CacheControl`.
#[derive(Debug)]
pub struct IterMut<'a> {
    inner: slice::IterMut<'a, Encoding>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut Encoding;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl ToHeaderValues for ContentEncoding {
    type Iter = option::IntoIter<HeaderValue>;
    fn to_header_values(&self) -> crate::Result<Self::Iter> {
        // A HeaderValue will always convert into itself.
        Ok(self.value().to_header_values().unwrap())
    }
}

impl Debug for ContentEncoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_list();
        for directive in &self.entries {
            list.entry(directive);
        }
        list.finish()
    }
}
