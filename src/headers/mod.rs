//! HTTP headers.

use std::collections::HashMap;
use std::convert::TryInto;
use std::iter::IntoIterator;

mod constants;
mod header_name;
mod header_value;
mod into_iter;
mod iter;
mod iter_mut;
mod names;
mod to_header_values;
mod values;

pub use constants::*;
pub use header_name::HeaderName;
pub use header_value::HeaderValue;
pub use into_iter::IntoIter;
pub use iter::Iter;
pub use iter_mut::IterMut;
pub use names::Names;
pub use to_header_values::ToHeaderValues;
pub use values::Values;

/// A collection of HTTP Headers.
#[derive(Debug, Clone)]
pub struct Headers {
    pub(crate) headers: HashMap<HeaderName, Vec<HeaderValue>>,
}

impl Headers {
    /// Create a new instance.
    pub(crate) fn new() -> Self {
        Self {
            headers: HashMap::new(),
        }
    }

    /// Insert a header into the headers.
    ///
    /// Not that this will replace all header values for a given header name.
    /// If you wish to add header values for a header name that already exists
    /// use `Headers::append`
    pub fn insert(
        &mut self,
        name: impl TryInto<HeaderName>,
        values: impl ToHeaderValues,
    ) -> crate::Result<Option<Vec<HeaderValue>>> {
        let name = name
            .try_into()
            .map_err(|_| crate::format_err!("Could not convert into header name"))?;
        let values: Vec<HeaderValue> = values.to_header_values()?.collect();
        Ok(self.headers.insert(name, values))
    }

    /// Append a header to the headers.
    ///
    /// Unlike `insert` this function will not override the contents of a header, but insert a
    /// header if there aren't any. Or else append to the existing list of headers.
    pub fn append(
        &mut self,
        name: impl TryInto<HeaderName>,
        values: impl ToHeaderValues,
    ) -> crate::Result<()> {
        let name = name
            .try_into()
            .map_err(|_| crate::format_err!("Could not convert into header name"))?;
        match self.get_mut(&name) {
            Some(headers) => {
                let mut values: Vec<HeaderValue> = values.to_header_values()?.collect();
                headers.append(&mut values);
            }
            None => {
                self.insert(name, values)?;
            }
        }
        Ok(())
    }

    /// Get a reference to a header.
    pub fn get(&self, name: &HeaderName) -> Option<&Vec<HeaderValue>> {
        self.headers.get(name)
    }

    /// Get a mutable reference to a header.
    pub fn get_mut(&mut self, name: &HeaderName) -> Option<&mut Vec<HeaderValue>> {
        self.headers.get_mut(name)
    }

    /// Remove a header.
    pub fn remove(&mut self, name: &HeaderName) -> Option<Vec<HeaderValue>> {
        self.headers.remove(name)
    }

    /// An iterator visiting all header pairs in arbitrary order.
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            inner: self.headers.iter(),
        }
    }

    /// An iterator visiting all header pairs in arbitrary order, with mutable references to the
    /// values.
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut {
            inner: self.headers.iter_mut(),
        }
    }

    /// An iterator visiting all header names in arbitrary order.
    pub fn names(&self) -> Names<'_> {
        Names {
            inner: self.headers.keys(),
        }
    }

    /// An iterator visiting all header values in arbitrary order.
    pub fn values(&self) -> Values<'_> {
        Values::new(self.headers.values())
    }
}

impl IntoIterator for Headers {
    type Item = (HeaderName, Vec<HeaderValue>);
    type IntoIter = IntoIter;

    /// Returns a iterator of references over the remaining items.
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.headers.into_iter(),
        }
    }
}

impl<'a> IntoIterator for &'a Headers {
    type Item = (&'a HeaderName, &'a Vec<HeaderValue>);
    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Headers {
    type Item = (&'a HeaderName, &'a mut Vec<HeaderValue>);
    type IntoIter = IterMut<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    const STATIC_HEADER: HeaderName = HeaderName::from_lowercase_str("hello");

    #[test]
    fn test_header_name_static_non_static() -> crate::Result<()> {
        let static_header = HeaderName::from_lowercase_str("hello");
        let non_static_header = HeaderName::from_str("hello")?;

        let mut headers = Headers::new();
        headers.append(STATIC_HEADER, "foo0")?;
        headers.append(static_header.clone(), "foo1")?;
        headers.append(non_static_header.clone(), "foo2")?;

        assert_eq!(
            &headers.get(&STATIC_HEADER).unwrap()[..],
            &["foo0", "foo1", "foo2",][..]
        );

        assert_eq!(
            &headers.get(&static_header).unwrap()[..],
            &["foo0", "foo1", "foo2",][..]
        );

        assert_eq!(
            &headers.get(&non_static_header).unwrap()[..],
            &["foo0", "foo1", "foo2",][..]
        );

        Ok(())
    }
}
