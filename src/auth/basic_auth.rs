use crate::errors::AuthError;
use crate::headers::{HeaderName, HeaderValue, Headers, AUTHORIZATION};
use crate::{
    auth::{AuthenticationScheme, Authorization},
    headers::Header,
};

/// HTTP Basic authorization.
///
/// # Specifications
///
/// - [RFC7617](https://tools.ietf.org/html/rfc7617)
///
/// # Examples
///
/// ```
/// # fn main() -> http_types::Result<()> {
/// #
/// use http_types::Response;
/// use http_types::auth::{AuthenticationScheme, BasicAuth};
///
/// let username = "nori";
/// let password = "secret_fish!!";
/// let authz = BasicAuth::new(username, password);
///
/// let mut res = Response::new(200);
/// res.insert_header(&authz, &authz);
///
/// let authz = BasicAuth::from_headers(res)?.unwrap();
///
/// assert_eq!(authz.username(), username);
/// assert_eq!(authz.password(), password);
/// #
/// # Ok(()) }
/// ```
#[derive(Debug)]
pub struct BasicAuth {
    username: String,
    password: String,
}

impl BasicAuth {
    /// Create a new instance of `BasicAuth`.
    pub fn new<U, P>(username: U, password: P) -> Self
    where
        U: AsRef<str>,
        P: AsRef<str>,
    {
        let username = username.as_ref().to_owned();
        let password = password.as_ref().to_owned();
        Self { username, password }
    }

    /// Create a new instance from headers.
    pub fn from_headers(headers: impl AsRef<Headers>) -> crate::Result<Option<Self>> {
        let auth = match Authorization::from_headers(headers)? {
            Some(auth) => auth,
            None => return Ok(None),
        };

        let scheme = auth.scheme();
        internal_ensure!(
            matches!(scheme, AuthenticationScheme::Basic),
            AuthError::SchemeUnexpected(AuthenticationScheme::Basic, scheme.to_string())
        );
        Self::from_credentials(auth.credentials()).map(Some)
    }

    /// Create a new instance from the base64 encoded credentials.
    pub fn from_credentials(credentials: impl AsRef<[u8]>) -> crate::Result<Self> {
        let bytes = base64::decode(credentials).map_err(|_| {
            AuthError::CredentialsInvalid(AuthenticationScheme::Basic, "invalid base64")
        })?;
        let credentials = String::from_utf8(bytes).map_err(|_| {
            AuthError::CredentialsInvalid(AuthenticationScheme::Basic, "invalid utf8 from base64")
        })?;

        let mut iter = credentials.splitn(2, ':');
        let username = iter.next();
        let password = iter.next();

        let (username, password) = match (username, password) {
            (Some(username), Some(password)) => (username.to_string(), password.to_string()),
            (Some(_), None) => {
                return Err(AuthError::CredentialsInvalid(
                    AuthenticationScheme::Basic,
                    "missing password",
                )
                .into())
            }
            (None, _) => {
                return Err(AuthError::CredentialsInvalid(
                    AuthenticationScheme::Basic,
                    "missing username",
                )
                .into())
            }
        };

        Ok(Self { username, password })
    }

    /// Get the username.
    pub fn username(&self) -> &str {
        self.username.as_str()
    }

    /// Get the password.
    pub fn password(&self) -> &str {
        self.password.as_str()
    }
}

impl Header for BasicAuth {
    fn header_name(&self) -> HeaderName {
        AUTHORIZATION
    }

    fn header_value(&self) -> HeaderValue {
        let scheme = AuthenticationScheme::Basic;
        let credentials = base64::encode(format!("{}:{}", self.username, self.password));
        let auth = Authorization::new(scheme, credentials);
        auth.header_value()
    }
}

#[cfg(test)]
mod test {
    use crate::headers::Headers;
    use crate::StatusCode;

    use super::*;

    #[test]
    fn smoke() -> crate::Result<()> {
        let username = "nori";
        let password = "secret_fish!!";
        let authz = BasicAuth::new(username, password);

        let mut headers = Headers::new();
        authz.apply_header(&mut headers);

        let authz = BasicAuth::from_headers(headers)?.unwrap();

        assert_eq!(authz.username(), username);
        assert_eq!(authz.password(), password);
        Ok(())
    }

    #[test]
    fn bad_request_on_parse_error() {
        let mut headers = Headers::new();
        headers
            .insert(AUTHORIZATION, "<nori ate the tag. yum.>")
            .unwrap();
        let err = BasicAuth::from_headers(headers).unwrap_err();
        assert_eq!(err.associated_status_code(), Some(StatusCode::BadRequest));
    }
}
