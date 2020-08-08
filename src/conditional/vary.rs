//! Apply the HTTP method if the ETag matches.

use crate::conditional::VaryDirective;
use crate::headers::{HeaderName, HeaderValue, Headers, ToHeaderValues, VARY};

use std::convert::TryInto;
use std::fmt::{self, Debug, Write};
use std::iter::Iterator;
use std::option;
use std::slice;
use std::str::FromStr;

/// Apply the HTTP method if the ETag matches.
///
/// # Specifications
///
/// - [RFC 7231, section 7.1.4: Vary](https://tools.ietf.org/html/rfc7231#section-7.1.4)
///
/// # Examples
///
/// ```
/// # fn main() -> http_types::Result<()> {
/// #
/// use http_types::Response;
/// use http_types::conditional::Vary;
///
/// let mut entries = Vary::new();
/// entries.push("User-Agent")?;
/// entries.push("Accept-Encoding")?;
///
/// let mut res = Response::new(200);
/// entries.apply(&mut res);
///
/// let entries = Vary::from_headers(res)?.unwrap();
/// let mut entries = entries.iter();
/// assert_eq!(entries.next().unwrap(), "User-Agent");
/// assert_eq!(entries.next().unwrap(), "Accept-Encoding");
/// #
/// # Ok(()) }
/// ```
pub struct Vary {
    entries: Vec<VaryDirective>,
}

impl Vary {
    /// Create a new instance of `Vary`.
    pub fn new() -> Self {
        Self { entries: vec![] }
    }

    /// Create a new instance from headers.
    pub fn from_headers(headers: impl AsRef<Headers>) -> crate::Result<Option<Self>> {
        let mut entries = vec![];
        let headers = match headers.as_ref().get(VARY) {
            Some(headers) => headers,
            None => return Ok(None),
        };

        for value in headers {
            for part in value.as_str().trim().split(',') {
                let entry = VaryDirective::from_str(part)?;
                entries.push(entry);
            }
        }

        Ok(Some(Self { entries }))
    }

    /// Sets the `If-Match` header.
    pub fn apply(&self, mut headers: impl AsMut<Headers>) {
        headers.as_mut().insert(VARY, self.value());
    }

    /// Get the `HeaderName`.
    pub fn name(&self) -> HeaderName {
        VARY
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
    pub fn push(
        &mut self,
        directive: impl TryInto<VaryDirective, Error = crate::Error>,
    ) -> crate::Result<()> {
        self.entries.push(directive.try_into()?);
        Ok(())
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

impl IntoIterator for Vary {
    type Item = VaryDirective;
    type IntoIter = IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.entries.into_iter(),
        }
    }
}

impl<'a> IntoIterator for &'a Vary {
    type Item = &'a VaryDirective;
    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Vary {
    type Item = &'a mut VaryDirective;
    type IntoIter = IterMut<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/// A borrowing iterator over entries in `Vary`.
#[derive(Debug)]
pub struct IntoIter {
    inner: std::vec::IntoIter<VaryDirective>,
}

impl Iterator for IntoIter {
    type Item = VaryDirective;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

/// A lending iterator over entries in `Vary`.
#[derive(Debug)]
pub struct Iter<'a> {
    inner: slice::Iter<'a, VaryDirective>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a VaryDirective;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

/// A mutable iterator over entries in `Vary`.
#[derive(Debug)]
pub struct IterMut<'a> {
    inner: slice::IterMut<'a, VaryDirective>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut VaryDirective;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl ToHeaderValues for Vary {
    type Iter = option::IntoIter<HeaderValue>;
    fn to_header_values(&self) -> crate::Result<Self::Iter> {
        // A HeaderValue will always convert into itself.
        Ok(self.value().to_header_values().unwrap())
    }
}

impl Debug for Vary {
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
    use crate::conditional::Vary;
    use crate::Response;

    #[test]
    fn smoke() -> crate::Result<()> {
        let mut entries = Vary::new();
        entries.push("User-Agent")?;
        entries.push("Accept-Encoding")?;

        let mut res = Response::new(200);
        entries.apply(&mut res);

        let entries = Vary::from_headers(res)?.unwrap();
        let mut entries = entries.iter();
        assert_eq!(entries.next().unwrap(), "User-Agent");
        assert_eq!(entries.next().unwrap(), "Accept-Encoding");
        Ok(())
    }
}
