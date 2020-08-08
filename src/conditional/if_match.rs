use crate::conditional::MatchDirective;
use crate::headers::{HeaderName, HeaderValue, Headers, ToHeaderValues, IF_MATCH};

use std::fmt::{self, Debug, Write};
use std::iter::Iterator;
use std::option;
use std::slice;

/// A Match-Control header.
///
/// # Examples
///
/// ```
/// # fn main() -> http_types::Result<()> {
/// #
/// use http_types::Response;
/// use http_types::conditional::{IfMatch, ETag};
///
/// let mut entries = IfMatch::new();
/// entries.push(ETag::new("0xcafebeef".to_string()));
/// entries.push(ETag::new("0xbeefcafe".to_string()));
///
/// let mut res = Response::new(200);
/// entries.apply(&mut res);
///
/// let entries = IfMatch::from_headers(res)?.unwrap();
/// let mut entries = entries.iter();
/// assert_eq!(entries.next().unwrap(), ETag::new("0xcafebeef".to_string()));
/// assert_eq!(entries.next().unwrap(), ETag::new("0xbeefcafe".to_string()));
/// #
/// # Ok(()) }
/// ```
pub struct IfMatch {
    entries: Vec<MatchDirective>,
}

impl IfMatch {
    /// Create a new instance of `IfMatch`.
    pub fn new() -> Self {
        Self { entries: vec![] }
    }

    /// Create a new instance from headers.
    pub fn from_headers(headers: impl AsRef<Headers>) -> crate::Result<Option<Self>> {
        let mut entries = vec![];
        let headers = match headers.as_ref().get(IF_MATCH) {
            Some(headers) => headers,
            None => return Ok(None),
        };

        for value in headers {
            for part in value.as_str().trim().split(',') {
                // Try and parse a directive from a str. If the directive is
                // unkown we skip it.
                if let Some(entry) = MatchDirective::from_str(part)? {
                    entries.push(entry);
                }
            }
        }

        Ok(Some(Self { entries }))
    }

    /// Sets the `If-Match` header.
    pub fn apply(&self, mut headers: impl AsMut<Headers>) {
        headers.as_mut().insert(IF_MATCH, self.value());
    }

    /// Get the `HeaderName`.
    pub fn name(&self) -> HeaderName {
        IF_MATCH
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
    pub fn push(&mut self, directive: impl Into<MatchDirective>) {
        self.entries.push(directive.into());
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

impl IntoIterator for IfMatch {
    type Item = MatchDirective;
    type IntoIter = IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.entries.into_iter(),
        }
    }
}

impl<'a> IntoIterator for &'a IfMatch {
    type Item = &'a MatchDirective;
    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut IfMatch {
    type Item = &'a mut MatchDirective;
    type IntoIter = IterMut<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/// A borrowing iterator over entries in `IfMatch`.
#[derive(Debug)]
pub struct IntoIter {
    inner: std::vec::IntoIter<MatchDirective>,
}

impl Iterator for IntoIter {
    type Item = MatchDirective;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

/// A lending iterator over entries in `IfMatch`.
#[derive(Debug)]
pub struct Iter<'a> {
    inner: slice::Iter<'a, MatchDirective>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a MatchDirective;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

/// A mutable iterator over entries in `IfMatch`.
#[derive(Debug)]
pub struct IterMut<'a> {
    inner: slice::IterMut<'a, MatchDirective>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut MatchDirective;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl ToHeaderValues for IfMatch {
    type Iter = option::IntoIter<HeaderValue>;
    fn to_header_values(&self) -> crate::Result<Self::Iter> {
        // A HeaderValue will always convert into itself.
        Ok(self.value().to_header_values().unwrap())
    }
}

impl Debug for IfMatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_list();
        for directive in &self.entries {
            list.entry(directive);
        }
        list.finish()
    }
}

#[cfg(test)]
mod test {
    use crate::conditional::{ETag, IfMatch};
    use crate::Response;

    #[test]
    fn smoke() -> crate::Result<()> {
        let mut entries = IfMatch::new();
        entries.push(ETag::new("0xcafebeef".to_string()));
        entries.push(ETag::new("0xbeefcafe".to_string()));

        let mut res = Response::new(200);
        entries.apply(&mut res);

        let entries = IfMatch::from_headers(res)?.unwrap();
        let mut entries = entries.iter();
        assert_eq!(entries.next().unwrap(), ETag::new("0xcafebeef".to_string()));
        assert_eq!(entries.next().unwrap(), ETag::new("0xbeefcafe".to_string()));
        Ok(())
    }
}
