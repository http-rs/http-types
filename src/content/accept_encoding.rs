use crate::content::EncodingProposal;
use crate::headers::{HeaderName, HeaderValue, Headers, ToHeaderValues, ACCEPT_ENCODING};

use std::fmt::{self, Debug};
use std::option;
use std::slice;

/// An Accept-Encoding header.
pub struct AcceptEncoding {
    entries: Vec<EncodingProposal>,
}

impl AcceptEncoding {
    /// Create a new instance of `AcceptEncoding`.
    pub fn new() -> Self {
        Self { entries: vec![] }
    }

    /// Create an instance of `AcceptEncoding` from a `Headers` instance.
    pub fn from_headers(headers: impl AsRef<Headers>) -> crate::Result<Option<Self>> {
        let mut entries = vec![];
        let headers = match headers.as_ref().get(ACCEPT_ENCODING) {
            Some(headers) => headers,
            None => return Ok(None),
        };

        for value in headers {
            for part in value.as_str().trim().split(',') {
                // Try and parse a directive from a str. If the directive is
                // unkown we skip it.
                if let Some(entry) = EncodingProposal::from_str(part)? {
                    entries.push(entry);
                }
            }
        }

        Ok(Some(Self { entries }))
    }

    /// Push a directive into the list of entries.
    pub fn push(&mut self, prop: EncodingProposal) {
        self.entries.push(prop);
    }

    /// Insert a `HeaderName` + `HeaderValue` pair into a `Headers` instance.
    pub fn apply(&self, mut headers: impl AsMut<Headers>) {
        headers.as_mut().insert(ACCEPT_ENCODING, self.value());
    }

    /// Get the `HeaderName`.
    pub fn name(&self) -> HeaderName {
        ACCEPT_ENCODING
    }

    /// Get the `HeaderValue`.
    pub fn value(&self) -> HeaderValue {
        todo!();
    }

    /// An iterator visiting all entries.
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            inner: self.entries.iter(),
        }
    }

    /// An iterator visiting all entries.
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut {
            inner: self.entries.iter_mut(),
        }
    }
}

impl IntoIterator for AcceptEncoding {
    type Item = EncodingProposal;
    type IntoIter = IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.entries.into_iter(),
        }
    }
}

impl<'a> IntoIterator for &'a AcceptEncoding {
    type Item = &'a EncodingProposal;
    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut AcceptEncoding {
    type Item = &'a mut EncodingProposal;
    type IntoIter = IterMut<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/// A borrowing iterator over entries in `AcceptEncoding`.
#[derive(Debug)]
pub struct IntoIter {
    inner: std::vec::IntoIter<EncodingProposal>,
}

impl Iterator for IntoIter {
    type Item = EncodingProposal;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

/// A lending iterator over entries in `AcceptEncoding`.
#[derive(Debug)]
pub struct Iter<'a> {
    inner: slice::Iter<'a, EncodingProposal>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a EncodingProposal;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

/// A mutable iterator over entries in `AcceptEncoding`.
#[derive(Debug)]
pub struct IterMut<'a> {
    inner: slice::IterMut<'a, EncodingProposal>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut EncodingProposal;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl ToHeaderValues for AcceptEncoding {
    type Iter = option::IntoIter<HeaderValue>;
    fn to_header_values(&self) -> crate::Result<Self::Iter> {
        // A HeaderValue will always convert into itself.
        Ok(self.value().to_header_values().unwrap())
    }
}

impl Debug for AcceptEncoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_list();
        for directive in &self.entries {
            list.entry(directive);
        }
        list.finish()
    }
}
