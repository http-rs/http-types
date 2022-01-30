//! Client header advertising which languages the client is able to understand.

use crate::content::LanguageProposal;
use crate::headers::{Header, HeaderValue, Headers, ACCEPT_LANGUAGE};

use std::fmt::{self, Debug, Write};
use std::slice;

/// Client header advertising which languages the client is able to understand.
pub struct AcceptLanguage {
    wildcard: bool,
    entries: Vec<LanguageProposal>,
}

impl AcceptLanguage {
    /// Create a new instance of `AcceptLanguage`.
    pub fn new() -> Self {
        Self {
            entries: vec![],
            wildcard: false,
        }
    }

    /// Create an instance of `AcceptLanguage` from a `Headers` instance.
    pub fn from_headers(headers: impl AsRef<Headers>) -> crate::Result<Option<Self>> {
        let mut entries = vec![];
        let headers = match headers.as_ref().get(ACCEPT_LANGUAGE) {
            Some(headers) => headers,
            None => return Ok(None),
        };

        let mut wildcard = false;

        for value in headers {
            for part in value.as_str().trim().split(',') {
                let part = part.trim();

                if part.is_empty() {
                    continue;
                } else if part == "*" {
                    wildcard = true;
                    continue;
                }

                let entry = LanguageProposal::from_str(part)?;
                entries.push(entry);
            }
        }

        Ok(Some(Self { wildcard, entries }))
    }

    /// Push a directive into the list of entries.
    pub fn push(&mut self, prop: impl Into<LanguageProposal>) {
        self.entries.push(prop.into())
    }

    /// Returns `true` if a wildcard directive was passed.
    pub fn wildcard(&self) -> bool {
        self.wildcard
    }

    /// Set the wildcard directive.
    pub fn set_wildcard(&mut self, wildcard: bool) {
        self.wildcard = wildcard
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

impl Header for AcceptLanguage {
    fn header_name(&self) -> crate::headers::HeaderName {
        ACCEPT_LANGUAGE
    }

    fn header_value(&self) -> crate::headers::HeaderValue {
        let mut output = String::new();
        for (n, directive) in self.entries.iter().enumerate() {
            let directive: HeaderValue = directive.clone().into();
            match n {
                0 => write!(output, "{}", directive).unwrap(),
                _ => write!(output, ", {}", directive).unwrap(),
            };
        }

        if self.wildcard {
            match output.len() {
                0 => write!(output, "*").unwrap(),
                _ => write!(output, ", *").unwrap(),
            };
        }

        // SAFETY: the internal string is validated to be ASCII.
        unsafe { HeaderValue::from_bytes_unchecked(output.into()) }
    }
}

/// A borrowing iterator over entries in `AcceptLanguage`.
#[derive(Debug)]
pub struct IntoIter {
    inner: std::vec::IntoIter<LanguageProposal>,
}

impl Iterator for IntoIter {
    type Item = LanguageProposal;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

/// A lending iterator over entries in `AcceptLanguage`.
#[derive(Debug)]
pub struct Iter<'a> {
    inner: slice::Iter<'a, LanguageProposal>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a LanguageProposal;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

/// A mutable iterator over entries in `AcceptLanguage`.
#[derive(Debug)]
pub struct IterMut<'a> {
    inner: slice::IterMut<'a, LanguageProposal>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut LanguageProposal;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl Debug for AcceptLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_list();
        for directive in &self.entries {
            list.entry(directive);
        }
        list.finish()
    }
}

impl IntoIterator for AcceptLanguage {
    type Item = LanguageProposal;
    type IntoIter = IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.entries.into_iter(),
        }
    }
}

impl<'a> IntoIterator for &'a AcceptLanguage {
    type Item = &'a LanguageProposal;
    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut AcceptLanguage {
    type Item = &'a mut LanguageProposal;
    type IntoIter = IterMut<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Response;

    #[test]
    fn smoke() -> crate::Result<()> {
        let lang = LanguageProposal::new("en-CA", Some(1.0)).unwrap();
        let mut accept = AcceptLanguage::new();
        accept.push(lang.clone());

        let mut headers = Response::new(200);
        accept.apply_header(&mut headers);

        let accept = AcceptLanguage::from_headers(headers)?.unwrap();
        assert_eq!(accept.iter().next().unwrap(), &lang);
        Ok(())
    }

    #[test]
    fn wildcard() -> crate::Result<()> {
        let mut accept = AcceptLanguage::new();
        accept.set_wildcard(true);

        let mut headers = Response::new(200);
        accept.apply_header(&mut headers);

        let accept = AcceptLanguage::from_headers(headers)?.unwrap();
        assert!(accept.wildcard());
        Ok(())
    }
}