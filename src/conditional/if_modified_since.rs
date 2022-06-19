use crate::headers::{Field, FieldName, FieldValue, Fields, IF_MODIFIED_SINCE};
use crate::utils::{fmt_http_date, parse_http_date};

use std::fmt::Debug;

use std::time::SystemTime;

/// Apply the HTTP method if the entity has been modified after the given
/// date.
///
/// # Specifications
///
/// - [RFC 7232, section 3.3: If-Modified-Since](https://tools.ietf.org/html/rfc7232#section-3.3)
///
/// # Examples
///
/// ```
/// # fn main() -> http_types::Result<()> {
/// #
/// use http_types::Response;
/// use http_types::conditional::IfModifiedSince;
/// use std::time::{SystemTime, Duration};
///
/// let time = SystemTime::now() + Duration::from_secs(5 * 60);
/// let expires = IfModifiedSince::new(time);
///
/// let mut res = Response::new(200);
/// res.insert_header(&expires, &expires);
///
/// let expires = IfModifiedSince::from_headers(res)?.unwrap();
///
/// // HTTP dates only have second-precision.
/// let elapsed = time.duration_since(expires.modified())?;
/// assert_eq!(elapsed.as_secs(), 0);
/// #
/// # Ok(()) }
/// ```
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct IfModifiedSince {
    instant: SystemTime,
}

impl IfModifiedSince {
    /// Create a new instance of `IfModifiedSince`.
    pub fn new(instant: SystemTime) -> Self {
        Self { instant }
    }

    /// Returns the last modification time listed.
    pub fn modified(&self) -> SystemTime {
        self.instant
    }

    /// Create an instance of `IfModifiedSince` from a `Headers` instance.
    pub fn from_headers(headers: impl AsRef<Fields>) -> crate::Result<Option<Self>> {
        let headers = match headers.as_ref().get(IF_MODIFIED_SINCE) {
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

impl Field for IfModifiedSince {
    const FIELD_NAME: FieldName = IF_MODIFIED_SINCE;
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
        let expires = IfModifiedSince::new(time);

        let mut headers = Fields::new();
        headers.insert_typed(expires);

        let expires = IfModifiedSince::from_headers(headers)?.unwrap();

        // HTTP dates only have second-precision
        let elapsed = time.duration_since(expires.modified())?;
        assert_eq!(elapsed.as_secs(), 0);
        Ok(())
    }

    #[test]
    fn bad_request_on_parse_error() {
        let mut headers = Fields::new();
        headers
            .insert(IF_MODIFIED_SINCE, "<nori ate the tag. yum.>")
            .unwrap();
        let err = IfModifiedSince::from_headers(headers).unwrap_err();
        assert_eq!(err.status(), 400);
    }
}
