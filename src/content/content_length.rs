use crate::headers::{Field, FieldName, FieldValue, Headers, CONTENT_LENGTH};
use crate::Status;

/// The size of the entity-body, in bytes, sent to the recipient.
///
/// # Specifications
///
/// - [RFC 7230, section 3.3.2: Content-Length](https://tools.ietf.org/html/rfc7230#section-3.3.2)
///
/// # Examples
///
/// ```
/// # fn main() -> http_types::Result<()> {
/// #
/// use http_types::Response;
/// use http_types::content::{ContentLength};
///
/// let content_len = ContentLength::new(12);
///
/// let mut res = Response::new(200);
/// res.insert_header(&content_len, &content_len);
///
/// let content_len = ContentLength::from_headers(res)?.unwrap();
/// assert_eq!(content_len.len(), 12);
/// #
/// # Ok(()) }
/// ```
#[derive(Debug)]
pub struct ContentLength {
    length: u64,
}

#[allow(clippy::len_without_is_empty)]
impl ContentLength {
    /// Create a new instance.
    pub fn new(length: u64) -> Self {
        Self { length }
    }

    /// Create a new instance from headers.
    pub fn from_headers(headers: impl AsRef<Headers>) -> crate::Result<Option<Self>> {
        let headers = match headers.as_ref().get(CONTENT_LENGTH) {
            Some(headers) => headers,
            None => return Ok(None),
        };

        // If we successfully parsed the header then there's always at least one
        // entry. We want the last entry.
        let value = headers.iter().last().unwrap();
        let length = value.as_str().trim().parse::<u64>().status(400)?;
        Ok(Some(Self { length }))
    }

    /// Get the content length.
    pub fn len(&self) -> u64 {
        self.length
    }

    /// Set the content length.
    pub fn set_len(&mut self, len: u64) {
        self.length = len;
    }
}

impl Field for ContentLength {
    fn field_name(&self) -> FieldName {
        CONTENT_LENGTH
    }
    fn field_value(&self) -> FieldValue {
        let output = format!("{}", self.length);

        // SAFETY: the internal string is validated to be ASCII.
        unsafe { FieldValue::from_bytes_unchecked(output.into()) }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::headers::Headers;

    #[test]
    fn smoke() -> crate::Result<()> {
        let content_len = ContentLength::new(12);

        let mut headers = Headers::new();
        content_headers.insert(len);

        let content_len = ContentLength::from_headers(headers)?.unwrap();
        assert_eq!(content_len.len(), 12);
        Ok(())
    }

    #[test]
    fn bad_request_on_parse_error() {
        let mut headers = Headers::new();
        headers
            .insert(CONTENT_LENGTH, "<nori ate the tag. yum.>")
            .unwrap();
        let err = ContentLength::from_headers(headers).unwrap_err();
        assert_eq!(err.status(), 400);
    }
}
