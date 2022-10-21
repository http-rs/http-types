use crate::auth::AuthenticationScheme;
use crate::errors::AuthError;
use crate::headers::{Header, HeaderName, HeaderValue, Headers, AUTHORIZATION};

/// Credentials to authenticate a user agent with a server.
///
/// # Specifications
///
/// - [RFC 7235, section 4.2: Authorization](https://tools.ietf.org/html/rfc7235#section-4.2)
///
/// # Examples
///
/// ```
/// # fn main() -> http_types::Result<()> {
/// #
/// use http_types::Response;
/// use http_types::auth::{AuthenticationScheme, Authorization};
///
/// let scheme = AuthenticationScheme::Basic;
/// let credentials = "0xdeadbeef202020";
/// let authz = Authorization::new(scheme, credentials.into());
///
/// let mut res = Response::new(200);
/// res.insert_header(&authz, &authz);
///
/// let authz = Authorization::from_headers(res)?.unwrap();
///
/// assert_eq!(authz.scheme(), AuthenticationScheme::Basic);
/// assert_eq!(authz.credentials(), credentials);
/// #
/// # Ok(()) }
/// ```
#[derive(Debug)]
pub struct Authorization {
    scheme: AuthenticationScheme,
    credentials: String,
}

impl Authorization {
    /// Create a new instance of `Authorization`.
    pub fn new(scheme: AuthenticationScheme, credentials: String) -> Self {
        Self {
            scheme,
            credentials,
        }
    }

    /// Create a new instance from headers.
    pub fn from_headers(headers: impl AsRef<Headers>) -> crate::Result<Option<Self>> {
        let headers = match headers.as_ref().get(AUTHORIZATION) {
            Some(headers) => headers,
            None => return Ok(None),
        };

        // If we successfully parsed the header then there's always at least one
        // entry. We want the last entry.
        let value = headers.iter().last().unwrap();

        let mut iter = value.as_str().splitn(2, ' ');
        let scheme = iter.next();
        let credential = iter.next();
        let (scheme, credentials) = match (scheme, credential) {
            (None, _) => return Err(AuthError::SchemeMissing.into()),
            (Some(_), None) => return Err(AuthError::CredentialsMissing.into()),
            (Some(scheme), Some(credentials)) => (scheme.parse()?, credentials.to_owned()),
        };

        Ok(Some(Self {
            scheme,
            credentials,
        }))
    }

    /// Get the authorization scheme.
    pub fn scheme(&self) -> AuthenticationScheme {
        self.scheme
    }

    /// Set the authorization scheme.
    pub fn set_scheme(&mut self, scheme: AuthenticationScheme) {
        self.scheme = scheme;
    }

    /// Get the authorization credentials.
    pub fn credentials(&self) -> &str {
        self.credentials.as_str()
    }

    /// Set the authorization credentials.
    pub fn set_credentials(&mut self, credentials: String) {
        self.credentials = credentials;
    }
}

impl Header for Authorization {
    fn header_name(&self) -> HeaderName {
        AUTHORIZATION
    }

    fn header_value(&self) -> HeaderValue {
        let output = format!("{} {}", self.scheme, self.credentials);

        // SAFETY: the internal string is validated to be ASCII.
        unsafe { HeaderValue::from_bytes_unchecked(output.into()) }
    }
}

#[cfg(test)]
mod test {
    use crate::headers::Headers;
    use crate::StatusCode;

    use super::*;

    #[test]
    fn smoke() -> crate::Result<()> {
        let scheme = AuthenticationScheme::Basic;
        let credentials = "0xdeadbeef202020";
        let authz = Authorization::new(scheme, credentials.into());

        let mut headers = Headers::new();
        authz.apply_header(&mut headers);

        let authz = Authorization::from_headers(headers)?.unwrap();

        assert_eq!(authz.scheme(), AuthenticationScheme::Basic);
        assert_eq!(authz.credentials(), credentials);
        Ok(())
    }

    #[test]
    fn bad_request_on_parse_error() {
        let mut headers = Headers::new();
        headers
            .insert(AUTHORIZATION, "<nori ate the tag. yum.>")
            .unwrap();
        let err = Authorization::from_headers(headers).unwrap_err();
        assert_eq!(err.associated_status_code(), Some(StatusCode::BadRequest));
    }
}
