use crate::auth::{AuthenticationScheme, Authorization};
use crate::format_err;
use crate::headers::{HeaderName, HeaderValue, Headers, AUTHORIZATION};
use crate::Status;

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
/// let authz = BasicAuth::new(username, Some(password));
///
/// let mut res = Response::new(200);
/// authz.apply(&mut res);
///
/// let authz = BasicAuth::from_headers(res)?.unwrap();
///
/// assert_eq!(authz.username(), username);
/// assert_eq!(authz.password(), Some(password));
/// #
/// # Ok(()) }
/// ```
#[derive(Debug)]
pub struct BasicAuth {
    username: String,
    password: Option<String>,
}

impl BasicAuth {
    /// Create a new instance of `BasicAuth`.
    pub fn new<U, P>(username: U, password: Option<P>) -> Self
    where
        U: AsRef<str>,
        P: AsRef<str>,
    {
        let username = username.as_ref().to_owned();
        let password = password.map(|p| p.as_ref().to_owned());
        Self { username, password }
    }

    /// Create a new instance from headers.
    pub fn from_headers(headers: impl AsRef<Headers>) -> crate::Result<Option<Self>> {
        let auth = match Authorization::from_headers(headers)? {
            Some(auth) => auth,
            None => return Ok(None),
        };

        let scheme = auth.scheme();
        if !matches!(scheme, AuthenticationScheme::Basic) {
            let mut err = format_err!("Expected basic auth scheme found `{}`", scheme);
            err.set_status(400);
            return Err(err);
        }

        let bytes = base64::decode(auth.credentials()).status(400)?;
        let credentials = String::from_utf8(bytes).status(400)?;

        let mut iter = credentials.splitn(2, ':');
        let username = iter.next();
        let password = iter.next();

        let (username, password) = match (username, password) {
            (Some(username), Some(password)) => (username.to_string(), Some(password.to_string())),
            (Some(username), None) => (username.to_string(), None),
            (None, _) => {
                let mut err = format_err!("Expected basic auth to contain a username");
                err.set_status(400);
                return Err(err);
            }
        };

        Ok(Some(Self { username, password }))
    }

    /// Sets the header.
    pub fn apply(&self, mut headers: impl AsMut<Headers>) {
        headers.as_mut().insert(self.name(), self.value());
    }

    /// Get the `HeaderName`.
    pub fn name(&self) -> HeaderName {
        AUTHORIZATION
    }

    /// Get the `HeaderValue`.
    pub fn value(&self) -> HeaderValue {
        let scheme = AuthenticationScheme::Basic;
        let credentials = match self.password.as_ref() {
            Some(password) => base64::encode(format!("{}:{}", self.username, password)),
            None => base64::encode(self.username.clone()),
        };
        let auth = Authorization::new(scheme, credentials);
        auth.value()
    }

    /// Get the username.
    pub fn username(&self) -> &str {
        self.username.as_str()
    }

    /// Get the password.
    pub fn password(&self) -> Option<&str> {
        self.password.as_deref()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::headers::Headers;

    #[test]
    fn smoke() -> crate::Result<()> {
        let username = "nori";
        let password = "secret_fish!!";
        let authz = BasicAuth::new(username, Some(password));

        let mut headers = Headers::new();
        authz.apply(&mut headers);

        let authz = BasicAuth::from_headers(headers)?.unwrap();

        assert_eq!(authz.username(), username);
        assert_eq!(authz.password(), Some(password));
        Ok(())
    }

    #[test]
    fn bad_request_on_parse_error() -> crate::Result<()> {
        let mut headers = Headers::new();
        headers.insert(AUTHORIZATION, "<nori ate the tag. yum.>");
        let err = BasicAuth::from_headers(headers).unwrap_err();
        assert_eq!(err.status(), 400);
        Ok(())
    }
}
