use std::ops::Deref;

use crate::headers::{FieldName, FieldValue};

/// A trait representing a [`HeaderName`] and [`HeaderValue`] pair.
#[doc(alias = "Header")]
#[doc(alias = "FieldHeader")]
pub trait Field
where
    Self: Sized,
{
    /// The header's name.
    fn field_name(&self) -> FieldName;

    /// Access the header's value.
    fn field_value(&self) -> FieldValue;
}

/// Conversion into a [`Field`].
#[doc(alias = "IntoHeader")]
pub trait IntoField {
    /// What type are we converting into?
    type IntoField: Field;

    /// Convert into a `Field`.
    fn into_field(self) -> Self::IntoField;
}

impl Field for (FieldName, FieldValue) {
    fn field_name(&self) -> FieldName {
        self.0
    }

    fn field_value(&self) -> FieldValue {
        self.1
    }
}

impl<'a, T: Field> Field for &'a T {
    fn field_name(&self) -> FieldName {
        self.deref().field_name()
    }

    fn field_value(&self) -> FieldValue {
        self.deref().field_value()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn header_from_strings() {
        let strings = ("Content-Length", "12");
        assert_eq!(strings.header_name(), "Content-Length");
        assert_eq!(strings.header_value(), "12");
    }
}
