use crate::headers::{Field, FieldName, FieldValue, Fields, LAST_MODIFIED};
use crate::utils::{fmt_http_date, parse_http_date};

use std::fmt::Debug;

use std::time::SystemTime;

/// The last modification date of a resource.
///
/// # Specifications
///
/// - [RFC 7232, section 2.2: Last-Modified](https://tools.ietf.org/html/rfc7232#section-2.2)
///
/// # Examples
///
/// ```
/// # fn main() -> http_types::Result<()> {
/// #
/// use http_types::Response;
/// use http_types::conditional::LastModified;
/// use std::time::{SystemTime, Duration};
///
/// let time = SystemTime::now() + Duration::from_secs(5 * 60);
/// let last_modified = LastModified::new(time);
///
/// let mut res = Response::new(200);
/// res.insert_header(&last_modified, &last_modified);
///
/// let last_modified = LastModified::from_headers(res)?.unwrap();
///
/// // HTTP dates only have second-precision.
/// let elapsed = time.duration_since(last_modified.modified())?;
/// assert_eq!(elapsed.as_secs(), 0);
/// #
/// # Ok(()) }
/// ```
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct LastModified {
    instant: SystemTime,
}

impl LastModified {
    /// Create a new instance of `LastModified`.
    pub fn new(instant: SystemTime) -> Self {
        Self { instant }
    }

    /// Returns the last modification time listed.
    pub fn modified(&self) -> SystemTime {
        self.instant
    }

    /// Create an instance of `LastModified` from a `Headers` instance.
    pub fn from_headers(headers: impl AsRef<Fields>) -> crate::Result<Option<Self>> {
        let headers = match headers.as_ref().get(LAST_MODIFIED) {
            Some(headers) => headers,
            None => return Ok(None),
        };

        // If we successfully parsed the header then there's always at least one
        // entry. We want the last entry.
        let header = headers.iter().last().unwrap();

        let instant = parse_http_date(header.as_str())?;
        Ok(Some(Self { instant }))
    }
}

impl Field for LastModified {
    const FIELD_NAME: FieldName = LAST_MODIFIED;
    fn field_value(&self) -> FieldValue {
        let output = fmt_http_date(self.instant);

        // SAFETY: the internal string is validated to be ASCII.
        unsafe { FieldValue::from_bytes_unchecked(output.into()) }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::headers::Fields;
    use std::time::Duration;

    #[test]
    fn smoke() -> crate::Result<()> {
        let time = SystemTime::now() + Duration::from_secs(5 * 60);
        let last_modified = LastModified::new(time);

        let mut headers = Fields::new();
        headers.insert_typed(last_modified);

        let last_modified = LastModified::from_headers(headers)?.unwrap();

        // HTTP dates only have second-precision
        let elapsed = time.duration_since(last_modified.modified())?;
        assert_eq!(elapsed.as_secs(), 0);
        Ok(())
    }

    #[test]
    fn bad_request_on_parse_error() {
        let mut headers = Fields::new();
        headers
            .insert(LAST_MODIFIED, "<nori ate the tag. yum.>")
            .unwrap();
        let err = LastModified::from_headers(headers).unwrap_err();
        assert_eq!(err.status(), 400);
    }
}
