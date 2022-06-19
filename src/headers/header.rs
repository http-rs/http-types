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

    /// Create a field from its parts.
    // TODO: move this to a separate trait.
    fn from_field_pair(name: FieldName, value: FieldValue) -> Result<Self, ()>;
}

mod void {
    use core::marker::PhantomData;

    enum Void {}

    impl Copy for Void {}

    impl Clone for Void {
        fn clone(&self) -> Self {
            match *self {}
        }
    }

    pub struct MustBeStr<T>(PhantomData<T>, Void);

    impl<T> Copy for MustBeStr<T> {}

    impl<T> Clone for MustBeStr<T> {
        fn clone(&self) -> Self {
            *self
        }
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
