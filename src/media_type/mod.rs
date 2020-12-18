//! IANA Media Types.
//!
//! [Read more](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types).

mod constants;
mod parse;

pub use constants::*;

use std::borrow::Cow;
use std::fmt::{self, Debug, Display};
use std::option;
use std::str::FromStr;

use crate::headers::{HeaderValue, ToHeaderValues};

use infer::Infer;

/// An IANA MIME media type.
///
/// ```
/// use http_types::MediaType;
/// use std::str::FromStr;
///
/// let media_type = MediaType::from_str("text/html;charset=utf-8").unwrap();
/// assert_eq!(media_type.essence(), "text/html");
/// assert_eq!(media_type.param("charset").unwrap(), "utf-8");
/// ```
// NOTE: we cannot statically initialize Strings with values yet, so we keep dedicated static
// fields for the static strings.
#[derive(Clone)]
pub struct MediaType {
    pub(crate) essence: String,
    pub(crate) basetype: String,
    pub(crate) subtype: String,
    pub(crate) static_essence: Option<&'static str>,
    pub(crate) static_basetype: Option<&'static str>,
    pub(crate) static_subtype: Option<&'static str>,
    pub(crate) params: Option<ParamKind>,
}

impl MediaType {
    /// Sniff the media type from a byte slice.
    pub fn sniff(bytes: &[u8]) -> crate::Result<Self> {
        let info = Infer::new();
        let media_type = match info.get(&bytes) {
            Some(info) => info.mime,
            None => crate::bail!("Could not sniff the media type"),
        };
        MediaType::from_str(&media_type)
    }

    /// Guess the media type from a file extension
    pub fn from_extension(extension: impl AsRef<str>) -> Option<Self> {
        match extension.as_ref() {
            "html" => Some(HTML),
            "js" | "mjs" | "jsonp" => Some(JAVASCRIPT),
            "json" => Some(JSON),
            "css" => Some(CSS),
            "svg" => Some(SVG),
            "xml" => Some(XML),
            _ => None,
        }
    }

    /// Access the MediaType's `type` value.
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

    /// Access the MediaType's `subtype` value.
    pub fn subtype(&self) -> &str {
        if let Some(subtype) = self.static_subtype {
            &subtype
        } else {
            &self.subtype
        }
    }

    /// Access the MediaType's `essence` value.
    pub fn essence(&self) -> &str {
        if let Some(essence) = self.static_essence {
            &essence
        } else {
            &self.essence
        }
    }

    /// Get a reference to a param.
    pub fn param(&self, name: impl Into<ParamName>) -> Option<&ParamValue> {
        let name: ParamName = name.into();
        self.params
            .as_ref()
            .map(|inner| match inner {
                ParamKind::Vec(v) => v
                    .iter()
                    .find_map(|(k, v)| if k == &name { Some(v) } else { None }),
                ParamKind::Utf8 => match name {
                    ParamName(Cow::Borrowed("charset")) => Some(&ParamValue(Cow::Borrowed("utf8"))),
                    _ => None,
                },
            })
            .flatten()
    }
}

impl PartialEq<MediaType> for MediaType {
    fn eq(&self, other: &MediaType) -> bool {
        let left = match self.static_essence {
            Some(essence) => essence,
            None => &self.essence,
        };
        let right = match other.static_essence {
            Some(essence) => essence,
            None => &other.essence,
        };
        left == right
    }
}

impl Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        parse::format(self, f)
    }
}

impl Debug for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(essence) = self.static_essence {
            Debug::fmt(essence, f)
        } else {
            Debug::fmt(&self.essence, f)
        }
    }
}

impl FromStr for MediaType {
    type Err = crate::Error;

    /// Create a new `MediaType`.
    ///
    /// Follows the [WHATWG MIME parsing algorithm](https://media_typesniff.spec.whatwg.org/#parsing-a-media_type-type).
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse::parse(s)
    }
}

impl<'a> From<&'a str> for MediaType {
    fn from(value: &'a str) -> Self {
        Self::from_str(value).unwrap()
    }
}

impl ToHeaderValues for MediaType {
    type Iter = option::IntoIter<HeaderValue>;

    fn to_header_values(&self) -> crate::Result<Self::Iter> {
        let media_type = self.clone();
        let header: HeaderValue = media_type.into();

        // A HeaderValue will always convert into itself.
        Ok(header.to_header_values().unwrap())
    }
}
/// A parameter name.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParamName(Cow<'static, str>);

impl ParamName {
    /// Get the name as a `&str`
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for ParamName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl FromStr for ParamName {
    type Err = crate::Error;

    /// Create a new `HeaderName`.
    ///
    /// This checks it's valid ASCII, and lowercases it.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        crate::ensure!(s.is_ascii(), "String slice should be valid ASCII");
        Ok(ParamName(Cow::Owned(s.to_ascii_lowercase())))
    }
}

impl<'a> From<&'a str> for ParamName {
    fn from(value: &'a str) -> Self {
        Self::from_str(value).unwrap()
    }
}

/// A parameter value.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParamValue(Cow<'static, str>);

impl ParamValue {
    /// Get the value as a `&str`
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for ParamValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<'a> PartialEq<&'a str> for ParamValue {
    fn eq(&self, other: &&'a str) -> bool {
        &self.0 == other
    }
}

impl PartialEq<str> for ParamValue {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

/// This is a hack that allows us to mark a trait as utf8 during compilation. We
/// can remove this once we can construct HashMap during compilation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ParamKind {
    Utf8,
    Vec(Vec<(ParamName, ParamValue)>),
}
