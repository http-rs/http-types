use std::borrow::Cow;
use std::io;
use std::iter;
use std::option;
use std::slice;

use crate::headers::{Field, FieldValue, FieldValues, Values};

/// A trait for objects which can be converted or resolved to one or more `HeaderValue`s.
pub trait ToFieldValues {
    /// Returned iterator over header values which this type may correspond to.
    type Iter: Iterator<Item = FieldValue>;

    /// Converts this object to an iterator of resolved `HeaderValues`.
    fn to_field_values(&self) -> crate::Result<Self::Iter>;
}

impl<T: Field> ToFieldValues for T {
    type Iter = option::IntoIter<FieldValue>;

    fn to_field_values(&self) -> crate::Result<Self::Iter> {
        Ok(Some(self.field_value()).into_iter())
    }
}

impl ToFieldValues for FieldValue {
    type Iter = option::IntoIter<FieldValue>;

    fn to_field_values(&self) -> crate::Result<Self::Iter> {
        Ok(Some(self.clone()).into_iter())
    }
}

impl<'a> ToFieldValues for &'a FieldValues {
    type Iter = iter::Cloned<Values<'a>>;

    fn to_field_values(&self) -> crate::Result<Self::Iter> {
        Ok(self.iter().cloned())
    }
}

impl<'a> ToFieldValues for &'a [FieldValue] {
    type Iter = iter::Cloned<slice::Iter<'a, FieldValue>>;

    fn to_field_values(&self) -> crate::Result<Self::Iter> {
        Ok(self.iter().cloned())
    }
}

impl<'a> ToFieldValues for &'a str {
    type Iter = option::IntoIter<FieldValue>;

    fn to_field_values(&self) -> crate::Result<Self::Iter> {
        let value = self
            .parse()
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        Ok(Some(value).into_iter())
    }
}

impl ToFieldValues for String {
    type Iter = option::IntoIter<FieldValue>;

    fn to_field_values(&self) -> crate::Result<Self::Iter> {
        self.as_str().to_field_values()
    }
}

impl ToFieldValues for &String {
    type Iter = option::IntoIter<FieldValue>;

    fn to_field_values(&self) -> crate::Result<Self::Iter> {
        self.as_str().to_field_values()
    }
}

impl ToFieldValues for Cow<'_, str> {
    type Iter = option::IntoIter<FieldValue>;

    fn to_field_values(&self) -> crate::Result<Self::Iter> {
        self.as_ref().to_field_values()
    }
}
