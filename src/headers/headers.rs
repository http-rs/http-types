//! HTTP headers.

use std::collections::HashMap;
use std::convert::Into;
use std::fmt::{self, Debug};
use std::iter::IntoIterator;
use std::ops::Index;
use std::str::FromStr;

use crate::headers::{
    Field, FieldName, FieldValues, IntoIter, Iter, IterMut, Names, ToFieldValues, Values,
};

use super::FieldValue;

/// A collection of HTTP Fields.
///
/// Fields are never manually constructed, but are part of `Request`,
/// `Response`, and `Trailers`. Each of these types implements `AsRef<Fields>`
/// and `AsMut<Fields>` so functions that want to modify headers can be generic
/// over either of these traits.
///
/// # Examples
///
/// ```
/// use http_types::{Response, StatusCode};
///
/// let mut res = Response::new(StatusCode::Ok);
/// res.insert_header("hello", "foo0");
/// assert_eq!(res["hello"], "foo0");
/// ```
#[derive(Clone)]
pub struct Fields {
    pub(crate) headers: HashMap<FieldName, FieldValues>,
}

impl Fields {
    /// Create a new instance.
    pub(crate) fn new() -> Self {
        Self {
            headers: HashMap::new(),
        }
    }

    /// Insert a header into the headers.
    ///
    /// Not that this will replace all header values for a given header name.
    pub fn insert(&mut self, name: FieldName, value: FieldValue) -> Option<FieldValue> {
        self.headers.insert(name, value)
    }

    /// Get a reference to a header.
    pub fn get(&self, name: FieldName) -> Option<&FieldValues> {
        self.headers.get(&name)
    }

    /// Get a mutable reference to a header.
    pub fn get_mut(&mut self, name: impl Into<FieldName>) -> Option<&mut FieldValues> {
        self.headers.get_mut(&name.into())
    }

    /// Remove a header.
    pub fn remove(&mut self, name: impl Into<FieldName>) -> Option<FieldValues> {
        self.headers.remove(&name.into())
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

impl Index<FieldName> for Fields {
    type Output = FieldValues;

    /// Returns a reference to the value corresponding to the supplied name.
    ///
    /// # Panics
    ///
    /// Panics if the name is not present in `Fields`.
    #[inline]
    fn index(&self, name: FieldName) -> &FieldValues {
        self.get(name).expect("no entry found for name")
    }
}

impl Index<&str> for Fields {
    type Output = FieldValues;

    /// Returns a reference to the value corresponding to the supplied name.
    ///
    /// # Panics
    ///
    /// Panics if the name is not present in `Fields`.
    #[inline]
    fn index(&self, name: &str) -> &FieldValues {
        let name = FieldName::from_str(name).expect("string slice needs to be valid ASCII");
        self.get(name).expect("no entry found for name")
    }
}

impl IntoIterator for Fields {
    type Item = (FieldName, FieldValues);
    type IntoIter = IntoIter;

    /// Returns a iterator of references over the remaining items.
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.headers.into_iter(),
        }
    }
}

impl<'a> IntoIterator for &'a Fields {
    type Item = (&'a FieldName, &'a FieldValues);
    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Fields {
    type Item = (&'a FieldName, &'a mut FieldValues);
    type IntoIter = IterMut<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl Debug for Fields {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.headers.iter()).finish()
    }
}

impl AsRef<Fields> for Fields {
    fn as_ref(&self) -> &Fields {
        self
    }
}

impl AsMut<Fields> for Fields {
    fn as_mut(&mut self) -> &mut Fields {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    const STATIC_HEADER: FieldName = FieldName::from_lowercase_str("hello");

    #[test]
    fn test_header_name_static_non_static() -> crate::Result<()> {
        let static_header = FieldName::from_lowercase_str("hello");
        let non_static_header = FieldName::from_str("hello")?;

        let mut headers = Fields::new();
        headers.append(STATIC_HEADER, "foo0").unwrap();
        headers.append(static_header.clone(), "foo1").unwrap();
        headers.append(non_static_header.clone(), "foo2").unwrap();

        assert_eq!(headers[STATIC_HEADER], ["foo0", "foo1", "foo2",][..]);
        assert_eq!(headers[static_header], ["foo0", "foo1", "foo2",][..]);
        assert_eq!(headers[non_static_header], ["foo0", "foo1", "foo2",][..]);

        Ok(())
    }

    #[test]
    fn index_into_headers() {
        let mut headers = Fields::new();
        headers.insert("hello", "foo0").unwrap();
        assert_eq!(headers["hello"], "foo0");
        assert_eq!(headers.get("hello").unwrap(), "foo0");
    }

    #[test]
    fn test_debug_single() {
        let mut headers = Fields::new();
        headers.insert("single", "foo0").unwrap();
        assert_eq!(format!("{:?}", headers), r#"{"single": "foo0"}"#);
    }

    #[test]
    fn test_debug_multiple() {
        let mut headers = Fields::new();
        headers.append("multi", "foo0").unwrap();
        headers.append("multi", "foo1").unwrap();
        assert_eq!(format!("{:?}", headers), r#"{"multi": ["foo0", "foo1"]}"#);
    }
}
