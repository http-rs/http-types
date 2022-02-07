use crate::headers::{Header, HeaderName, HeaderValue, Headers};
use crate::Status;

use crate::headers::STRICT_TRANSPORT_SECURITY;
use std::time::Duration;

/// Inform browsers that the site should only be accessed using HTTPS.
///
/// # Specifications
///
/// - [RFC 6797, section 6.1: Strict-Transport-Security](https://www.rfc-editor.org/rfc/rfc6797#section-6.1)
#[derive(Debug)]
#[doc(alias = "hsts")]
pub struct StrictTransportSecurity {
    max_age: Duration,
    include_subdomains: bool,
    preload: bool,
}

impl Default for StrictTransportSecurity {
    /// Defaults to 1 year with "preload" enabled, passing the minimum requirements to
    /// qualify for inclusion in browser's HSTS preload lists.
    /// [Read more](https://hstspreload.org/)
    fn default() -> Self {
        Self {
            max_age: Duration::from_secs(31536000), // 1 year
            include_subdomains: false,
            preload: true,
        }
    }
}

impl StrictTransportSecurity {
    /// Create a new instance.
    pub fn new(duration: Duration) -> Self {
        Self {
            max_age: duration,
            include_subdomains: false,
            preload: false,
        }
    }
    /// Get a reference to the strict transport security's include subdomains.
    pub fn include_subdomains(&self) -> bool {
        self.include_subdomains
    }

    /// Set the strict transport security's include subdomains.
    pub fn set_include_subdomains(&mut self, include_subdomains: bool) {
        self.include_subdomains = include_subdomains;
    }

    /// Get a reference to the strict transport security's preload.
    pub fn preload(&self) -> bool {
        self.preload
    }

    /// Set the strict transport security's preload.
    pub fn set_preload(&mut self, preload: bool) {
        self.preload = preload;
    }

    /// Get a reference to the strict transport security's max_age.
    pub fn max_age(&self) -> Duration {
        self.max_age
    }

    /// Set the strict transport security's max_age.
    pub fn set_max_age(&mut self, duration: Duration) {
        self.max_age = duration;
    }
}

impl Header for StrictTransportSecurity {
    fn header_name(&self) -> HeaderName {
        STRICT_TRANSPORT_SECURITY
    }

    fn header_value(&self) -> HeaderValue {
        let max_age = self.max_age.as_secs();
        let mut output = format!("max-age={}", max_age);
        if self.include_subdomains {
            output.push_str(";includeSubdomains");
        }
        if self.preload {
            output.push_str(";preload");
        }

        // SAFETY: the internal string is validated to be ASCII.
        unsafe { HeaderValue::from_bytes_unchecked(output.into()) }
    }
}

// TODO: move to new header traits
impl StrictTransportSecurity {
    /// Create a new instance from headers.
    pub fn from_headers(headers: impl AsRef<Headers>) -> crate::Result<Option<Self>> {
        let headers = match headers.as_ref().get(STRICT_TRANSPORT_SECURITY) {
            Some(headers) => headers,
            None => return Ok(None),
        };

        // If we successfully parsed the header then there's always at least one
        // entry. We want the last entry.
        let value = headers.iter().last().unwrap();

        let mut max_age = None;
        let mut include_subdomains = false;
        let mut preload = false;

        // Attempt to parse all values. If we don't recognize a directive, per
        // the spec we should just ignore it.
        for s in value.as_str().split(';') {
            let s = s.trim();
            if s == "includesubdomains" {
                include_subdomains = true;
            } else if s == "preload" {
                preload = true;
            } else {
                let (key, value) = match s.split_once("=") {
                    Some(kv) => kv,
                    None => continue, // We don't recognize the directive, continue.
                };

                if key == "max-age" {
                    let secs = value.parse::<u64>().status(400)?;
                    max_age = Some(Duration::from_secs(secs));
                }
            }
        }

        let max_age = match max_age {
            Some(max_age) => max_age,
            None => {
                return Err(crate::format_err_status!(
                    400,
                    "`Strict-Transport-Security` header did not contain a `max-age` directive",
                ));
            }
        };

        Ok(Some(Self {
            max_age,
            include_subdomains,
            preload,
        }))
    }
}

impl From<StrictTransportSecurity> for Duration {
    fn from(stc: StrictTransportSecurity) -> Self {
        stc.max_age
    }
}

impl From<Duration> for StrictTransportSecurity {
    fn from(duration: Duration) -> Self {
        Self::new(duration)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Response;
    use std::time::Duration;

    #[test]
    fn smoke() -> crate::Result<()> {
        let duration = Duration::from_secs(30);
        let stc = StrictTransportSecurity::new(duration);

        let mut headers = Response::new(200);
        headers.insert(stc);

        let stc = StrictTransportSecurity::from_headers(headers)?.unwrap();

        assert_eq!(stc.max_age(), duration);
        assert!(!stc.preload);
        assert!(!stc.include_subdomains);
        Ok(())
    }

    #[test]
    fn bad_request_on_parse_error() {
        let mut headers = Response::new(200);
        headers
            .insert_header(STRICT_TRANSPORT_SECURITY, "<nori ate the tag. yum.>")
            .unwrap();
        let err = StrictTransportSecurity::from_headers(headers).unwrap_err();
        assert_eq!(err.status(), 400);
    }

    #[test]
    fn no_panic_on_invalid_number() {
        let mut headers = Response::new(200);
        headers
            .insert_header(STRICT_TRANSPORT_SECURITY, "max-age=birds")
            .unwrap();
        let err = StrictTransportSecurity::from_headers(headers).unwrap_err();
        assert_eq!(err.status(), 400);
    }

    #[test]
    fn parse_optional_whitespace() {
        let mut headers = Response::new(200);
        headers
            .insert_header(STRICT_TRANSPORT_SECURITY, "max-age=30;     preload")
            .unwrap();
        let policy = StrictTransportSecurity::from_headers(headers)
            .unwrap()
            .unwrap();
        assert_eq!(policy.max_age, Duration::from_secs(30));
        assert!(policy.preload());
    }
}
