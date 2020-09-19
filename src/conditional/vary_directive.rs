use crate::headers::{HeaderName, HeaderValue};
use std::convert::TryFrom;
use std::str::FromStr;

/// An HeaderName-based match directive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VaryDirective {
    /// An HeaderName.
    HeaderName(HeaderName),
    /// Vary any resource.
    Wildcard,
}

// impl VaryDirective {
//     /// Create an instance from a string slice.
//     //
//     // This is a private method rather than a trait because we assume the
//     // input string is a single-value only. This is upheld by the calling
//     // function, but we cannot guarantee this to be true in the general
//     // sense.
//     pub(crate) fn from_str(s: &str) -> crate::Result<Option<Self>> {
//         let s = s.trim();

//         match s {
//             "*" => Ok(Some(VaryDirective::Wildcard)),
//             s => {
//                 HeaderName::from_string(s.into()).map(|name| Some(VaryDirective::HeaderName(name)))
//             }
//         }
//     }
// }

impl From<HeaderName> for VaryDirective {
    fn from(name: HeaderName) -> Self {
        Self::HeaderName(name)
    }
}

impl PartialEq<HeaderName> for VaryDirective {
    fn eq(&self, other: &HeaderName) -> bool {
        match self {
            Self::HeaderName(name) => name.eq(other),
            Self::Wildcard => false,
        }
    }
}

impl<'a> PartialEq<HeaderName> for &'a VaryDirective {
    fn eq(&self, other: &HeaderName) -> bool {
        match self {
            VaryDirective::HeaderName(name) => name.eq(other),
            VaryDirective::Wildcard => false,
        }
    }
}

impl From<VaryDirective> for HeaderValue {
    fn from(directive: VaryDirective) -> Self {
        match directive {
            VaryDirective::HeaderName(name) => unsafe {
                HeaderValue::from_bytes_unchecked(name.to_string().into_bytes())
            },
            VaryDirective::Wildcard => unsafe {
                HeaderValue::from_bytes_unchecked("*".to_string().into_bytes())
            },
        }
    }
}

impl FromStr for VaryDirective {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "*" => Ok(VaryDirective::Wildcard),
            s => Ok(VaryDirective::HeaderName(s.parse()?)),
        }
    }
}

impl<'a> TryFrom<&'a str> for VaryDirective {
    type Error = crate::Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        VaryDirective::from_str(value)
    }
}

impl PartialEq<str> for VaryDirective {
    fn eq(&self, other: &str) -> bool {
        match self {
            VaryDirective::Wildcard => "*" == other,
            VaryDirective::HeaderName(s) => s == other,
        }
    }
}

impl<'a> PartialEq<&'a str> for VaryDirective {
    fn eq(&self, other: &&'a str) -> bool {
        match self {
            VaryDirective::Wildcard => &"*" == other,
            VaryDirective::HeaderName(s) => s == other,
        }
    }
}
