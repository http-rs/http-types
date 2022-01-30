//! Client header advertising which languages the client is able to understand.

use crate::content::LanguageProposal;
use crate::headers::{Header, HeaderValue, Headers, ACCEPT_LANGUAGE};

use std::fmt::{self, Debug, Write};

/// Client header advertising which languages the client is able to understand.
pub struct AcceptLanguage {
    wildcard: bool,
    entries: Vec<LanguageProposal>,
}

impl AcceptLanguage {
    /// Create a new instance of `AcceptLanguage`.
    pub fn new() -> Self {
        Self {
            entries: vec![],
            wildcard: false,
        }
    }

    /// Create an instance of `AcceptLanguage` from a `Headers` instance.
    pub fn from_headers(headers: impl AsRef<Headers>) -> crate::Result<Option<Self>> {
        let mut entries = vec![];
        let headers = match headers.as_ref().get(ACCEPT_LANGUAGE) {
            Some(headers) => headers,
            None => return Ok(None),
        };

        let mut wildcard = false;

        for value in headers {
            for part in value.as_str().trim().split(',') {
                let part = part.trim();

                if part.is_empty() {
                    continue;
                } else if part == "*" {
                    wildcard = true;
                    continue;
                }

                let entry = LanguageProposal::from_str(part)?;
                entries.push(entry);
            }
        }

        Ok(Some(Self { wildcard, entries }))
    }
}

impl Header for AcceptLanguage {
    fn header_name(&self) -> crate::headers::HeaderName {
        ACCEPT_LANGUAGE
    }

    fn header_value(&self) -> crate::headers::HeaderValue {
        let mut output = String::new();
        for (n, directive) in self.entries.iter().enumerate() {
            let directive: HeaderValue = directive.clone().into();
            match n {
                0 => write!(output, "{}", directive).unwrap(),
                _ => write!(output, ", {}", directive).unwrap(),
            };
        }

        if self.wildcard {
            match output.len() {
                0 => write!(output, "*").unwrap(),
                _ => write!(output, ", *").unwrap(),
            };
        }

        // SAFETY: the internal string is validated to be ASCII.
        unsafe { HeaderValue::from_bytes_unchecked(output.into()) }
    }
}

impl Debug for AcceptLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = f.debug_list();
        for directive in &self.entries {
            list.entry(directive);
        }
        list.finish()
    }
}
