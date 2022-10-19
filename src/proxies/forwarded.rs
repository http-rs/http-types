use std::{borrow::Cow, collections::HashMap, net::IpAddr, ops::Deref};

use crate::{
    headers::{Header, HeaderName, HeaderValue, Headers, FORWARDED},
    parse_utils::{parse_quoted_string, parse_token, tchar},
};

// These constants are private because they are non-standard.
const X_FORWARDED_BY: HeaderName = HeaderName::from_lowercase_str("x-forwarded-by");
const X_FORWARDED_FOR: HeaderName = HeaderName::from_lowercase_str("x-forwarded-for");
const X_FORWARDED_HOST: HeaderName = HeaderName::from_lowercase_str("x-forwarded-host");
const X_FORWARDED_PROTO: HeaderName = HeaderName::from_lowercase_str("x-forwarded-proto");

/// Error type for parsing `Forwarded` headers.
#[derive(Debug, PartialEq, Eq)]
pub enum ForwardedError {
    /// Returned when parsing a [`ForwardedElement`] failed.
    ForwardedElementError(ForwardedElementError),
    /// Returned when the input string contained trailing data.
    TrailingData(String),
    /// Returned when the input contained multiple `X-Forwarded-*` headers when falling back.
    MultipleXForwardedHeaders(Vec<String>),
}

impl std::error::Error for ForwardedError {}

impl std::fmt::Display for ForwardedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ForwardedError::ForwardedElementError(err) => {
                write!(f, "Failed to parse ForwardedElement: {err}")
            }
            ForwardedError::TrailingData(data) => write!(f, "Input had trailing data: {data:?}"),
            ForwardedError::MultipleXForwardedHeaders(headers) => {
                let headers = headers.join(", ");
                write!(f, "Multiple X-Forwarded-* headers found: {}", headers)
            }
        }
    }
}

impl From<ForwardedElementError> for ForwardedError {
    fn from(err: ForwardedElementError) -> Self {
        ForwardedError::ForwardedElementError(err)
    }
}

/// A Rust representation of the [RFC 7329](https://www.rfc-editor.org/rfc/rfc7239#section-4)
/// `Forwarded` production.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Forwarded<'fe> {
    /// Ordered vector of `forwarded-element`.
    pub elements: Vec<ForwardedElement<'fe>>,
}

impl<'fe, 'input: 'fe> Forwarded<'fe> {
    /// Builds a new `Forwarded`.
    pub fn new() -> Self {
        Forwarded::default()
    }

    /// Parse a list of `Forwarded` HTTP headers into a borrowed `Forwarded` instance, with
    /// `X-Forwarded-*` fallback.
    ///
    /// # Fallback behaviour
    ///
    /// If no `Forwarded` HTTP header was set it falls back to trying to parse one of the supported
    /// kinds of `X-Forwarded-*` headers.  See
    /// [`from_x_forwarded_headers`](Self::from_x_forwarded_headers) for more information.
    ///
    /// # Examples
    /// ```rust
    /// # use http_types::{proxies::Forwarded, Method::Get, Request, Url, Result};
    /// # fn main() -> Result<()> {
    /// let mut request = Request::new(Get, Url::parse("http://_/")?);
    /// request.insert_header("X-Forwarded-For", "198.51.100.46");
    /// request.insert_header(
    ///     "Forwarded",
    ///     r#"for=192.0.2.43, for="[2001:db8:cafe::17]", for=unknown;proto=https"#
    /// );
    /// let forwarded = Forwarded::from_headers(&request)?.unwrap();
    /// //assert_eq!(forwarded.to_string(), r#"for=192.0.2.43, for="[2001:db8:cafe::17]", for=unknown;proto=https"#);
    /// # Ok(()) }
    /// ```
    ///
    /// ```rust
    /// # use http_types::{proxies::Forwarded, Method::Get, Request, Url, Result};
    /// # fn main() -> Result<()> {
    /// let mut request = Request::new(Get, Url::parse("http://_/")?);
    /// request.insert_header("X-Forwarded-For", "192.0.2.43, 2001:db8:cafe::17, unknown");
    /// let forwarded = Forwarded::from_headers(&request)?.unwrap();
    /// assert_eq!(forwarded.to_string(), r#"for=192.0.2.43, for="[2001:db8:cafe::17]", for=unknown"#);
    /// # Ok(()) }
    /// ```
    ///
    /// ```rust
    /// # use http_types::{proxies::{Forwarded, ForwardedError}, Method::Get, Request, Url, Result};
    /// # fn main() -> Result<()> {
    /// let mut request = Request::new(Get, Url::parse("http://_/")?);
    /// request.insert_header("X-Forwarded-For", "192.0.2.43, 2001:db8:cafe::17, unknown");
    /// request.insert_header("X-Forwarded-Proto", "https");
    /// assert_eq!(
    ///     Forwarded::from_headers(&request),
    ///     Err(ForwardedError::MultipleXForwardedHeaders(vec![
    ///         "x-forwarded-for".to_string(),
    ///         "x-forwarded-proto".to_string()
    ///     ])),
    /// );
    /// # Ok(()) }
    /// ```
    pub fn from_headers(
        headers: &'input impl AsRef<Headers>,
    ) -> Result<Option<Self>, ForwardedError> {
        if let Some(forwarded) = Self::from_forwarded_header(headers)? {
            Ok(Some(forwarded))
        } else {
            Self::from_x_forwarded_headers(headers)
        }
    }

    /// Parses list of `Forwarded` HTTP headers into a borrowed `Forwarded` instance.
    pub fn from_forwarded_header(
        headers: &'input impl AsRef<Headers>,
    ) -> Result<Option<Self>, ForwardedError> {
        let headers = if let Some(headers) = headers.as_ref().get(FORWARDED) {
            headers
        } else {
            return Ok(None);
        };

        let mut forwarded = Forwarded::new();
        for value in headers {
            let rest = forwarded.parse(value.as_str())?;
            if !rest.is_empty() {
                return Err(ForwardedError::TrailingData(rest.to_string()));
            }
        }

        Ok(Some(forwarded))
    }

    /// Attempt to parse non-standard `X-Forwarded-*` headers into a borrowed `Forwarded` instance.
    ///
    /// This will only attempt to do the conversion if only one kind of `X-Forwarded-*` header was
    /// specified since there is no way for us to know which order the headers were added in and at
    /// which steps.  This is in accordance with Section 7.4 of RFC 7239.
    ///
    /// # Supported headers
    ///
    /// - `X-Forwarded-By`
    /// - `X-Forwarded-For`
    /// - `X-Forwarded-Host`
    /// - `X-Forwarded-Proto`
    pub fn from_x_forwarded_headers(
        headers: &'input impl AsRef<Headers>,
    ) -> Result<Option<Self>, ForwardedError> {
        let headers = headers.as_ref();

        let mut found_headers = Vec::new();
        for header in [
            &X_FORWARDED_BY,
            &X_FORWARDED_FOR,
            &X_FORWARDED_HOST,
            &X_FORWARDED_PROTO,
        ] {
            if let Some(found) = headers.names().find(|h| h == &header) {
                found_headers.push(found.as_str().to_string());
            }
        }

        match found_headers.len() {
            0 => return Ok(None),
            1 => {}
            // If there were more than one kind of `X-Forwarded-*` header we shouldn't try to parse
            // them since there is no way to know in which order they were added and by which
            // proxies.  C.f. Section 7.4 of RFC 7239.
            _ => return Err(ForwardedError::MultipleXForwardedHeaders(found_headers)),
        }

        let mut forwarded = Forwarded::new();

        if let Some(values) = headers.get(X_FORWARDED_BY) {
            values.as_str().split(',').for_each(|value| {
                let value = value.trim();
                let value = match value.parse::<IpAddr>().ok() {
                    Some(IpAddr::V6(v6)) => format!("[{}]", v6).into(),
                    _ => value.into(),
                };
                forwarded.elements.push(ForwardedElement {
                    by: Some(value),
                    ..Default::default()
                });
            })
        }

        if let Some(values) = headers.get(X_FORWARDED_FOR) {
            values.as_str().split(',').for_each(|value| {
                let value = value.trim();
                let value = match value.parse::<IpAddr>().ok() {
                    Some(IpAddr::V6(v6)) => format!("[{}]", v6).into(),
                    _ => value.into(),
                };
                forwarded.elements.push(ForwardedElement {
                    r#for: Some(value),
                    ..Default::default()
                });
            })
        }

        if let Some(values) = headers.get(X_FORWARDED_HOST) {
            values.as_str().split(',').for_each(|value| {
                let value = value.trim();
                forwarded.elements.push(ForwardedElement {
                    host: Some(value.into()),
                    ..Default::default()
                });
            })
        }

        if let Some(values) = headers.get(X_FORWARDED_PROTO) {
            values.as_str().split(',').for_each(|value| {
                let value = value.trim();
                forwarded.elements.push(ForwardedElement {
                    proto: Some(value.into()),
                    ..Default::default()
                });
            })
        }

        Ok(Some(forwarded))
    }

    /// Parses a `Forwarded` HTTP header value into a borrowed `Forwarded` instance.
    pub fn parse(&mut self, input: &'input str) -> Result<&'input str, ForwardedError> {
        let (mut element, mut rest) = ForwardedElement::parse(input)?;
        self.elements.push(element);

        while rest.starts_with(',') {
            (element, rest) = ForwardedElement::parse(&rest[1..])?;
            self.elements.push(element);
        }

        Ok(rest)
    }

    /// Transform a borrowed `Forwarded` into an owned `Forwarded`.
    pub fn into_owned(self) -> Forwarded<'static> {
        Forwarded {
            elements: self
                .elements
                .into_iter()
                .map(|element| element.into_owned())
                .collect(),
        }
    }
}

impl<'fe> std::fmt::Display for Forwarded<'fe> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elements: Vec<_> = self
            .elements
            .iter()
            .map(ForwardedElement::to_string)
            .collect();

        write!(f, "{}", elements.join(", "))
    }
}

impl<'input> Header for Forwarded<'input> {
    fn header_name(&self) -> HeaderName {
        FORWARDED
    }

    fn header_value(&self) -> HeaderValue {
        let output = self.to_string();

        // SAFETY: the internal strings are validated to be ASCII.
        unsafe { HeaderValue::from_bytes_unchecked(output.into()) }
    }
}

/// Error type for parsing `ForwardedElement`s.
#[derive(Debug, PartialEq, Eq)]
pub enum ForwardedElementError {
    /// Returned when parsing a parameter name failed.
    ParameterParseError,
    /// Returned when parsing a parameter value failed.
    ValueParseError,
    /// Returned when the parser expected a specific character and got another one.
    UnexpectedCharacter(char, char),
    /// Returned when a `forwarded-element` contained the same parameter more than once.
    DuplicateParameter(String),
    /// Returned when trying to set a parameter key that isn't a valid token.
    NonTokenParameter,
    /// Returned when trying to set or parse a non-ASCII parameter value.
    NonAsciiValue,
}

impl std::error::Error for ForwardedElementError {}

impl std::fmt::Display for ForwardedElementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ForwardedElementError::ParameterParseError => write!(f, "Failed to parse parameter"),
            ForwardedElementError::ValueParseError => write!(f, "Failed to parse value"),
            ForwardedElementError::UnexpectedCharacter(expected, found) => {
                write!(f, "Expected character {expected:?}, found {found:?}")
            }
            ForwardedElementError::DuplicateParameter(parameter) => {
                write!(f, "Same parameter was found multiple times: {parameter:?}")
            }
            ForwardedElementError::NonTokenParameter => {
                write!(f, "Tried to set a parameter that wasn't a valid token")
            }
            ForwardedElementError::NonAsciiValue => {
                write!(f, "Failed set parameter to non-ASCII value")
            }
        }
    }
}

/// A Rust representation of the [RFC 7329](https://www.rfc-editor.org/rfc/rfc7239#section-4)
/// `forwarded-element` production.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ForwardedElement<'fe> {
    /// Identifies the user-agent facing interface of the proxy.
    by: Option<Cow<'fe, str>>,
    /// Identifies the node making the request to the proxy.
    r#for: Option<Cow<'fe, str>>,
    /// The host request header field as received by the proxy.
    host: Option<Cow<'fe, str>>,
    /// Indicates what protocol was used to make the request.
    proto: Option<Cow<'fe, str>>,
    /// Map of `Forwarded` header extension parameters.
    extensions: HashMap<Cow<'fe, str>, Cow<'fe, str>>,
}

impl<'fe, 'input: 'fe> ForwardedElement<'fe> {
    /// Parses a string conforming to the [RFC
    /// 7329](https://www.rfc-editor.org/rfc/rfc7239#section-4) `forwarded-element` ABNF production
    /// into a `ForwardedElement`
    pub fn parse(input: &'input str) -> Result<(Self, &'input str), ForwardedElementError> {
        let mut element = ForwardedElement::default();

        // Skip any potential whitespace ahead of the whole forwarded-element.
        let mut rest = skip_whitespace(input);

        rest = element.parse_forwarded_pair(rest)?;
        loop {
            match rest.chars().next() {
                Some(';') => rest = element.parse_forwarded_pair(&rest[1..])?,
                Some(',') => break,
                Some(c) => return Err(ForwardedElementError::UnexpectedCharacter(';', c)),
                None => break,
            }
        }

        Ok((element, rest))
    }

    fn parse_forwarded_pair(
        &mut self,
        input: &'input str,
    ) -> Result<&'input str, ForwardedElementError> {
        let (parameter, rest) =
            parse_token(input).ok_or(ForwardedElementError::ParameterParseError)?;

        let rest = match rest.chars().next() {
            Some('=') => &rest[1..],
            Some(c) => return Err(ForwardedElementError::UnexpectedCharacter('=', c)),
            None => return Err(ForwardedElementError::ParameterParseError),
        };

        let (value, rest) = parse_value(rest).ok_or(ForwardedElementError::ValueParseError)?;
        // SAFETY: all internal strings have to be valid ASCII.
        if !value.is_ascii() {
            return Err(ForwardedElementError::NonAsciiValue);
        }

        match rest.chars().next() {
            Some(',' | ';') => {}
            None => {}
            _ => return Err(ForwardedElementError::ValueParseError),
        }

        match parameter.to_ascii_lowercase().as_str() {
            "by" => {
                if self.by.is_some() {
                    return Err(ForwardedElementError::DuplicateParameter("by".into()));
                }
                self.by = Some(value);
            }
            "for" => {
                if self.r#for.is_some() {
                    return Err(ForwardedElementError::DuplicateParameter("for".into()));
                }
                self.r#for = Some(value);
            }
            "host" => {
                if self.host.is_some() {
                    return Err(ForwardedElementError::DuplicateParameter("host".into()));
                }
                self.host = Some(value);
            }
            "proto" => {
                if self.proto.is_some() {
                    return Err(ForwardedElementError::DuplicateParameter("proto".into()));
                }
                self.proto = Some(value);
            }
            _ => {
                if self.extensions.contains_key(&parameter) {
                    return Err(ForwardedElementError::DuplicateParameter(parameter.into()));
                }
                self.extensions.insert(parameter, value);
            }
        }

        Ok(rest)
    }

    /// Transforms a borrowed `ForwardedElement` into an owned `ForwardedElement.
    pub fn into_owned(self) -> ForwardedElement<'static> {
        ForwardedElement {
            by: self.by.map(|by| Cow::Owned(by.into_owned())),
            r#for: self.r#for.map(|r#for| Cow::Owned(r#for.into_owned())),
            host: self.host.map(|host| Cow::Owned(host.into_owned())),
            proto: self.proto.map(|proto| Cow::Owned(proto.into_owned())),
            extensions: self
                .extensions
                .into_iter()
                .map(|(property, value)| {
                    (
                        Cow::Owned(property.into_owned()),
                        Cow::Owned(value.into_owned()),
                    )
                })
                .collect(),
        }
    }

    /// Sets the `by` parameter value.
    pub fn set_by(&mut self, by: impl Into<Cow<'input, str>>) -> Result<(), ForwardedElementError> {
        let value = by.into();

        // SAFETY: all internal strings have to be valid ASCII.
        if !value.is_ascii() {
            return Err(ForwardedElementError::NonAsciiValue);
        }
        self.by = Some(value);
        Ok(())
    }

    /// Returns the `by` parameter value.
    pub fn by(&self) -> Option<&str> {
        self.by.as_deref()
    }

    /// Sets the `for` parameter value.
    pub fn set_for(
        &mut self,
        forwarded_for: impl Into<Cow<'input, str>>,
    ) -> Result<(), ForwardedElementError> {
        let value = forwarded_for.into();

        // SAFETY: all internal strings have to be valid ASCII.
        if !value.is_ascii() {
            return Err(ForwardedElementError::NonAsciiValue);
        }
        self.r#for = Some(value);
        Ok(())
    }

    /// Returns the `for` parameter value.
    pub fn r#for(&self) -> Option<&str> {
        self.r#for.as_deref()
    }

    /// Sets the `host` parameter value
    pub fn set_host(
        &mut self,
        host: impl Into<Cow<'input, str>>,
    ) -> Result<(), ForwardedElementError> {
        let value = host.into();

        // SAFETY: all internal strings have to be valid ASCII.
        if !value.is_ascii() {
            return Err(ForwardedElementError::NonAsciiValue);
        }
        self.host = Some(value);
        Ok(())
    }

    /// Returns the `host` parameter value.
    pub fn host(&self) -> Option<&str> {
        self.host.as_deref()
    }

    /// Sets the `proto` parameter value.
    pub fn set_proto(
        &mut self,
        proto: impl Into<Cow<'input, str>>,
    ) -> Result<(), ForwardedElementError> {
        let value = proto.into();

        // SAFETY: all internal strings have to be valid ASCII.
        if !value.is_ascii() {
            return Err(ForwardedElementError::NonAsciiValue);
        }
        self.proto = Some(value);
        Ok(())
    }

    /// Returns the `proto` parameter value.
    pub fn proto(&self) -> Option<&str> {
        self.proto.as_deref()
    }

    /// Sets an extension parameter value.
    pub fn set_extension(
        &mut self,
        parameter: impl Into<Cow<'input, str>>,
        value: impl Into<Cow<'input, str>>,
    ) -> Result<Option<Cow<'fe, str>>, ForwardedElementError> {
        let parameter = parameter.into();
        if !parameter.chars().all(tchar) {
            return Err(ForwardedElementError::NonTokenParameter);
        }

        // SAFETY: all internal strings have to be valid ASCII.
        let value = value.into();
        if !value.is_ascii() {
            return Err(ForwardedElementError::NonAsciiValue);
        }

        Ok(self.extensions.insert(parameter, value))
    }

    /// Returns an extension parameter value, if set.
    pub fn extension(
        &'fe self,
        parameter: impl Into<Cow<'input, str>>,
    ) -> Result<Option<&'fe str>, ForwardedElementError> {
        let parameter = parameter.into();

        // SAFETY: all internal strings have to be valid ASCII.
        if !parameter.chars().all(tchar) {
            return Err(ForwardedElementError::NonTokenParameter);
        }

        Ok(self.extensions.get(&parameter).map(|value| value.deref()))
    }

    /// Returns the `HashMap` of extension parameters.
    pub fn extensions(&self) -> &HashMap<Cow<'fe, str>, Cow<'fe, str>> {
        &self.extensions
    }
}

impl<'fe> std::fmt::Display for ForwardedElement<'fe> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut write_semicolon = {
            let mut first = true;
            move |f: &mut std::fmt::Formatter<'_>| -> std::fmt::Result {
                if first {
                    first = false;
                } else {
                    write!(f, ";")?;
                }
                Ok(())
            }
        };

        if let Some(by) = &self.by {
            write_semicolon(f)?;
            write!(f, "by={}", format_value(by.as_ref()))?;
        }

        if let Some(r#for) = &self.r#for {
            write_semicolon(f)?;
            write!(f, "for={}", format_value(r#for.as_ref()))?;
        }

        if let Some(host) = &self.host {
            write_semicolon(f)?;
            write!(f, "host={}", format_value(host.as_ref()))?;
        }

        if let Some(proto) = &self.proto {
            write_semicolon(f)?;
            write!(f, "proto={}", format_value(proto.as_ref()))?;
        }

        for (parameter, value) in self.extensions.iter() {
            write_semicolon(f)?;
            write!(f, "{}={}", parameter, format_value(value.as_ref()))?;
        }

        Ok(())
    }
}

fn parse_value(input: &str) -> Option<(Cow<'_, str>, &str)> {
    let (value, rest) = parse_token(input).or_else(|| parse_quoted_string(input))?;
    Some((value, skip_whitespace(rest)))
}

fn format_value(input: &str) -> Cow<'_, str> {
    if input.chars().all(tchar) {
        // If the value fully consists of token characters, write it out as-is.
        return input.into();
    }

    // Otherwise write out a quoted string.
    let mut out = String::from("\"");
    for c in input.chars() {
        match c {
            '"' | '\\' => {
                out.push('\\');
                out.push(c);
            }
            _ => out.push(c),
        }
    }
    out.push('"');
    out.into()
}

fn skip_whitespace(input: &str) -> &str {
    let mut rest = input;
    while rest.starts_with(' ') {
        rest = &rest[1..];
    }
    rest
}

#[cfg(test)]
mod tests {
    use super::*;

    mod forwarded {
        use url::Url;

        use crate::{Method, Request};

        use super::*;

        #[test]
        fn two_values_same_string() {
            let mut actual = Forwarded::default();

            let rest = actual
                .parse("by=192.0.2.2;for=192.0.2.1, for=192.0.2.3;by=192.0.2.4")
                .expect("Forwarded header value didn't parse");
            assert_eq!(rest, "");

            let expected = Forwarded {
                elements: vec![
                    ForwardedElement {
                        by: Some("192.0.2.2".into()),
                        r#for: Some("192.0.2.1".into()),
                        ..Default::default()
                    },
                    ForwardedElement {
                        by: Some("192.0.2.4".into()),
                        r#for: Some("192.0.2.3".into()),
                        ..Default::default()
                    },
                ],
            };
            assert_eq!(actual, expected);
        }

        #[test]
        fn two_separate_strings() {
            let mut actual = Forwarded::default();

            let rest = actual
                .parse("by=192.0.2.6;for=192.0.2.5;host=example.org;proto=https;something=another")
                .expect("Forwarded header value didn't parse");
            assert_eq!(rest, "");

            let rest = actual
                .parse("by=192.0.2.8;for=192.0.2.7;host=example.com;proto=http;bar=baz")
                .expect("Forwarded header value didn't parse");
            assert_eq!(rest, "");

            let mut extensions1 = HashMap::new();
            extensions1.insert("something".into(), "another".into());
            let mut extensions2 = HashMap::new();
            extensions2.insert("bar".into(), "baz".into());
            let expected = Forwarded {
                elements: vec![
                    ForwardedElement {
                        by: Some("192.0.2.6".into()),
                        r#for: Some("192.0.2.5".into()),
                        host: Some("example.org".into()),
                        proto: Some("https".into()),
                        extensions: extensions1,
                    },
                    ForwardedElement {
                        by: Some("192.0.2.8".into()),
                        r#for: Some("192.0.2.7".into()),
                        host: Some("example.com".into()),
                        proto: Some("http".into()),
                        extensions: extensions2,
                    },
                ],
            };
            assert_eq!(actual, expected);
        }

        #[test]
        fn x_forwarded_for() {
            let mut request = Request::new(Method::Get, Url::parse("http://_/").unwrap());
            request
                .append_header(X_FORWARDED_FOR, "192.0.2.43, 2001:db8:cafe::17")
                .unwrap();

            let forwarded = Forwarded::from_x_forwarded_headers(&request)
                .expect("Failed to parse headers")
                .expect("Found no headers");

            assert_eq!(
                forwarded,
                Forwarded {
                    elements: vec![
                        ForwardedElement {
                            r#for: Some("192.0.2.43".into()),
                            ..Default::default()
                        },
                        ForwardedElement {
                            r#for: Some("[2001:db8:cafe::17]".into()),
                            ..Default::default()
                        },
                    ],
                },
            );
        }

        #[test]
        fn multiple_x_forwarded_headers() {
            let mut request = Request::new(Method::Get, Url::parse("http://_/").unwrap());
            request
                .append_header(X_FORWARDED_FOR, "192.0.2.43, 2001:db8:cafe::17")
                .unwrap();
            request.append_header(X_FORWARDED_PROTO, "gopher").unwrap();
            let res =
                Forwarded::from_x_forwarded_headers(&request).expect_err("Parsing didn't fail");
            assert_eq!(
                res,
                ForwardedError::MultipleXForwardedHeaders(vec![
                    X_FORWARDED_FOR.to_string(),
                    X_FORWARDED_PROTO.to_string(),
                ])
            );
        }

        #[test]
        fn to_string() {
            let mut forwarded = Forwarded::default();
            forwarded
                .parse("by=192.0.2.2;for=192.0.2.1, for=192.0.2.3;by=192.0.2.4")
                .expect("Forwarded header value didn't parse");

            assert_eq!(
                forwarded.to_string(),
                "by=192.0.2.2;for=192.0.2.1, by=192.0.2.4;for=192.0.2.3",
            );
        }

        #[test]
        fn owned_can_outlive_request() {
            let forwarded = {
                let mut request = Request::new(Method::Get, Url::parse("http://_/").unwrap());
                request
                    .append_header("Forwarded", "for=for;by=by;host=host;proto=proto")
                    .unwrap();
                Forwarded::from_headers(&request)
                    .unwrap()
                    .unwrap()
                    .into_owned()
            };
            assert_eq!(forwarded.elements[0].by, Some("by".into()));
        }

        #[test]
        fn all_rfc_examples() {
            let examples = vec![
                (
                    r#"for="_gazonk""#,
                    vec![ForwardedElement {
                        r#for: Some("_gazonk".into()),
                        ..Default::default()
                    }],
                ),
                (
                    r#"For="[2001:db8:cafe::17]:4711""#,
                    vec![ForwardedElement {
                        r#for: Some("[2001:db8:cafe::17]:4711".into()),
                        ..Default::default()
                    }],
                ),
                (
                    r#" for=192.0.2.60;proto=http;by=203.0.113.43"#,
                    vec![ForwardedElement {
                        by: Some("203.0.113.43".into()),
                        r#for: Some("192.0.2.60".into()),
                        proto: Some("http".into()),
                        ..Default::default()
                    }],
                ),
                (
                    r#"for=192.0.2.43, for=198.51.100.17"#,
                    vec![
                        ForwardedElement {
                            r#for: Some("192.0.2.43".into()),
                            ..Default::default()
                        },
                        ForwardedElement {
                            r#for: Some("198.51.100.17".into()),
                            ..Default::default()
                        },
                    ],
                ),
                (
                    r#"for=_hidden, for=_SEVKISEK"#,
                    vec![
                        ForwardedElement {
                            r#for: Some("_hidden".into()),
                            ..Default::default()
                        },
                        ForwardedElement {
                            r#for: Some("_SEVKISEK".into()),
                            ..Default::default()
                        },
                    ],
                ),
                (
                    r#"for=192.0.2.43,for="[2001:db8:cafe::17]",for=unknown"#,
                    vec![
                        ForwardedElement {
                            r#for: Some("192.0.2.43".into()),
                            ..Default::default()
                        },
                        ForwardedElement {
                            r#for: Some("[2001:db8:cafe::17]".into()),
                            ..Default::default()
                        },
                        ForwardedElement {
                            r#for: Some("unknown".into()),
                            ..Default::default()
                        },
                    ],
                ),
                (
                    r#"for=192.0.2.43, for="[2001:db8:cafe::17]", for=unknown"#,
                    vec![
                        ForwardedElement {
                            r#for: Some("192.0.2.43".into()),
                            ..Default::default()
                        },
                        ForwardedElement {
                            r#for: Some("[2001:db8:cafe::17]".into()),
                            ..Default::default()
                        },
                        ForwardedElement {
                            r#for: Some("unknown".into()),
                            ..Default::default()
                        },
                    ],
                ),
                (
                    r#"for=192.0.2.43"#,
                    vec![ForwardedElement {
                        r#for: Some("192.0.2.43".into()),
                        ..Default::default()
                    }],
                ),
                (
                    r#"for="[2001:db8:cafe::17]", for=unknown"#,
                    vec![
                        ForwardedElement {
                            r#for: Some("[2001:db8:cafe::17]".into()),
                            ..Default::default()
                        },
                        ForwardedElement {
                            r#for: Some("unknown".into()),
                            ..Default::default()
                        },
                    ],
                ),
                (
                    r#"for=192.0.2.43, for="[2001:db8:cafe::17]""#,
                    vec![
                        ForwardedElement {
                            r#for: Some("192.0.2.43".into()),
                            ..Default::default()
                        },
                        ForwardedElement {
                            r#for: Some("[2001:db8:cafe::17]".into()),
                            ..Default::default()
                        },
                    ],
                ),
                (
                    r#"for=192.0.2.43, for=198.51.100.17;by=203.0.113.60;proto=http;host=example.com"#,
                    vec![
                        ForwardedElement {
                            r#for: Some("192.0.2.43".into()),
                            ..Default::default()
                        },
                        ForwardedElement {
                            by: Some("203.0.113.60".into()),
                            r#for: Some("198.51.100.17".into()),
                            host: Some("example.com".into()),
                            proto: Some("http".into()),
                            ..Default::default()
                        },
                    ],
                ),
            ];
            for (value, elements) in examples.into_iter() {
                let mut headers = Headers::new();
                headers.append("Forwarded", value).unwrap();

                let parsed = Forwarded::from_forwarded_header(&headers)
                    .expect(&format!("Failed while parsing {:?}", value))
                    .expect(&format!("No headers found while parsing {:?}", value));
                assert_eq!(parsed.elements, elements);
            }
        }
    }

    mod forwarded_element {
        use super::*;

        #[test]
        fn empty_input() {
            let res = ForwardedElement::parse("");
            assert_eq!(res, Err(ForwardedElementError::ParameterParseError));
        }

        #[test]
        fn duplicate_parameters() {
            let res = ForwardedElement::parse("for=bar;for=baz");
            assert_eq!(
                res,
                Err(ForwardedElementError::DuplicateParameter("for".into()))
            );
        }

        #[test]
        fn invalid_value() {
            let res = ForwardedElement::parse("for=bär");
            assert_eq!(res, Err(ForwardedElementError::ValueParseError));
        }

        #[test]
        fn space_within_element_is_invalid() {
            let res = ForwardedElement::parse("for=bar; by=baz");
            assert_eq!(res, Err(ForwardedElementError::ParameterParseError));
        }

        #[test]
        fn all_parameters() {
            let (res, rest) = ForwardedElement::parse(
                "by=192.0.2.1;for=192.0.2.2;host=example.org;proto=https;something=another",
            )
            .expect("String didn't parse as ForwardedElement");

            let mut extensions = HashMap::new();
            extensions.insert("something".into(), "another".into());
            assert_eq!(
                res,
                ForwardedElement {
                    by: Some("192.0.2.1".into()),
                    r#for: Some("192.0.2.2".into()),
                    host: Some("example.org".into()),
                    proto: Some("https".into()),
                    extensions,
                }
            );

            assert_eq!(rest, "");
        }

        #[test]
        fn to_string() {
            let mut extensions = HashMap::new();
            extensions.insert("something".into(), r#"some "thing""#.into());
            let element = ForwardedElement {
                by: Some("[2001:db8:cafe::17]:4711".into()),
                r#for: Some("[2001:db8:cafe::16]:2342".into()),
                host: Some("example.org".into()),
                proto: Some("https".into()),
                extensions,
            };

            println!("{}", &element.to_string());
            assert_eq!(
                element.to_string(),
                r#"by="[2001:db8:cafe::17]:4711";for="[2001:db8:cafe::16]:2342";host=example.org;proto=https;something="some \"thing\"""#
            );
        }

        #[test]
        fn non_ascii_value() {
            let element = ForwardedElement::parse(r#"for=for;by=by;host=host;proto="pröto""#)
                .expect_err("Didn't fail to parse non-ASCII parameter value");
            assert_eq!(element, ForwardedElementError::NonAsciiValue,);
        }
    }
}
