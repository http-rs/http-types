use crate::headers::{FieldName, FieldValue};

/// A trait representing a [`HeaderName`] and [`HeaderValue`] pair.
///
/// # Specifications
///
/// - [RFC 9110, section 5: Fields](https://www.rfc-editor.org/rfc/rfc9110.html#fields)
#[doc(alias = "Header")]
#[doc(alias = "FieldHeader")]
pub trait Field
where
    Self: Sized,
{
    /// The header's field name.
    const FIELD_NAME: FieldName;

    /// Access the header's value.
    fn field_value(&self) -> FieldValue;

    // /// Create a field from its parts.
    // // TODO: move this to a separate trait.
    // fn from_field_pair(name: FieldName, value: FieldValue) -> Result<Self, ()>;
}
