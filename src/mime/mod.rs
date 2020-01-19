//! IANA Media Types.
//!
//! [Read more](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types).

mod constants;
mod parse;

pub use constants::*;

use std::collections::HashMap;
use std::fmt::{self, Debug, Display};
use std::io;
use std::option;
use std::str::FromStr;

use crate::headers::{HeaderValue, ParseError, ToHeaderValues};
use crate::Error;
use crate::StatusCode;

use infer::Infer;

/// An IANA media type.
// NOTE: we cannot statically initialize Strings with values yet, so we keep dedicated static
// fields for the static strings.
#[derive(Clone)]
pub struct Mime {
    pub(crate) essence: String,
    pub(crate) basetype: String,
    pub(crate) subtype: String,
    pub(crate) static_essence: Option<&'static str>,
    pub(crate) static_basetype: Option<&'static str>,
    pub(crate) static_subtype: Option<&'static str>,
    pub(crate) parameters: Option<HashMap<String, String>>,
}

impl Mime {
    /// Sniff the mime type from a byte slice.
    pub fn sniff(bytes: &[u8]) -> crate::Result<Self> {
        let info = Infer::new();
        let mime = match info.get(&bytes) {
            Some(info) => info.mime,
            None => {
let err = io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Could not sniff the mime type",
                );
                return Err(Error::from_io(err, StatusCode::BadRequest))
            }
        };

        Ok(Self {
            essence: mime,
            static_essence: None,
            basetype: String::new(), // TODO: fill in.
            subtype: String::new(),  // TODO: fill in.
            static_basetype: None,   // TODO: fill in
            static_subtype: None,
            parameters: None, // TODO: fill in.
        })
    }

    /// Access the Mime's `type` value.
    ///
    /// According to the spec this method should be named `type`, but that's a reserved keyword in
    /// Rust so hence prefix with `base` instead.
    pub fn basetype(&self) -> &str {
        if let Some(basetype) = self.static_basetype {
            &basetype
        } else {
            &self.basetype
        }
    }

    /// Access the Mime's `subtype` value.
    pub fn subtype(&self) -> &str {
        if let Some(subtype) = self.static_subtype {
            &subtype
        } else {
            &self.subtype
        }
    }

    /// Access the Mime's `essence` value.
    pub fn essence(&self) -> &str {
        if let Some(essence) = self.static_essence {
            &essence
        } else {
            &self.essence
        }
    }

    /// Get a reference to a param.
    pub fn param(&self, s: &str) -> Option<&String> {
        self.parameters.as_ref().map(|hm| hm.get(s)).flatten()
    }

    /// Get a mutable reference to a param.
    pub fn param_mut(&mut self, s: &str) -> Option<&mut String> {
        self.parameters.as_mut().map(|hm| hm.get_mut(s)).flatten()
    }
}

impl Display for Mime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(essence) = self.static_essence {
            write!(f, "{}", essence)?
        } else {
            write!(f, "{}", &self.essence)?
        }
        if let Some(parameters) = &self.parameters {
            assert!(!parameters.is_empty());
            write!(f, "; ")?;
            for (i, (key, value)) in parameters.iter().enumerate() {
                write!(f, "{}={}", key, value)?;
                if i != parameters.len() - 1 {
                    write!(f, ",")?;
                }
            }
        }
        Ok(())
    }
}

impl Debug for Mime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(essence) = self.static_essence {
            Debug::fmt(essence, f)
        } else {
            Debug::fmt(&self.essence, f)
        }
    }
}

impl FromStr for Mime {
    type Err = ParseError;

    /// Create a new `HeaderName`.
    ///
    /// This checks it's valid ASCII, and lowercases it.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(ParseError::new());
        }
        Ok(Self {
            essence: s.to_ascii_lowercase(),
            static_essence: None,
            basetype: String::new(), // TODO: fill in.
            subtype: String::new(),  // TODO: fill in.
            static_basetype: None,   // TODO: fill in
            static_subtype: None,    // TODO: fill in
            parameters: None,        // TODO: fill in.
        })
    }
}

impl ToHeaderValues for Mime {
    type Iter = option::IntoIter<HeaderValue>;

    fn to_header_values(&self) -> crate::Result<Self::Iter> {
        let mime = self.clone();
        let header: HeaderValue = mime.into();

        // A HeaderValue will always convert into itself.
        Ok(header.to_header_values().unwrap())
    }
}
