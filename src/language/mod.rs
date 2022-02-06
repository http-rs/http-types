//! RFC 4647 Language Ranges.
//!
//! [Read more](https://datatracker.ietf.org/doc/html/rfc4647)

mod parse;

use crate::headers::HeaderValue;
use std::{
    borrow::Cow,
    fmt::{self, Display},
    slice,
    str::FromStr,
};

/// An RFC 4647 language range.
#[derive(Debug, Clone, PartialEq)]
pub struct LanguageRange {
    pub(crate) subtags: Vec<Cow<'static, str>>,
}

impl LanguageRange {
    /// An iterator visiting all entries.
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            inner: self.subtags.iter(),
        }
    }

    /// An iterator visiting all entries.
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut {
            inner: self.subtags.iter_mut(),
        }
    }
}

impl Display for LanguageRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tags = self.subtags.iter();
        if let Some(tag) = tags.next() {
            write!(f, "{}", tag)?;

            for tag in tags {
                write!(f, "-{}", tag)?;
            }
        }
        Ok(())
    }
}

/// A borrowing iterator over entries in `LanguageRange`.
#[derive(Debug)]
pub struct IntoIter {
    inner: std::vec::IntoIter<Cow<'static, str>>,
}

impl Iterator for IntoIter {
    type Item = Cow<'static, str>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

/// A lending iterator over entries in `LanguageRange`.
#[derive(Debug)]
pub struct Iter<'a> {
    inner: slice::Iter<'a, Cow<'static, str>>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Cow<'static, str>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

/// A mutable iterator over entries in `LanguageRange`.
#[derive(Debug)]
pub struct IterMut<'a> {
    inner: slice::IterMut<'a, Cow<'static, str>>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut Cow<'static, str>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl IntoIterator for LanguageRange {
    type Item = Cow<'static, str>;
    type IntoIter = IntoIter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.subtags.into_iter(),
        }
    }
}

impl<'a> IntoIterator for &'a LanguageRange {
    type Item = &'a Cow<'static, str>;
    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut LanguageRange {
    type Item = &'a mut Cow<'static, str>;
    type IntoIter = IterMut<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl From<LanguageRange> for HeaderValue {
    fn from(language: LanguageRange) -> Self {
        let s = language.to_string();
        unsafe { HeaderValue::from_bytes_unchecked(s.into_bytes()) }
    }
}

impl FromStr for LanguageRange {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse::parse(s)
    }
}

impl<'a> From<&'a str> for LanguageRange {
    fn from(value: &'a str) -> Self {
        Self::from_str(value).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_iter() -> crate::Result<()> {
        let range: LanguageRange = "en-CA".parse().unwrap();
        let subtags: Vec<_> = range.iter().collect();
        assert_eq!(&subtags, &["en", "CA"]);
        Ok(())
    }
}
