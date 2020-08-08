use crate::conditional::ETag;
use crate::headers::HeaderValue;

/// An Entity Tag based match directive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchDirective {
    /// An ETag.
    ETag(ETag),
    /// Match any resource.
    Wildcard,
}

impl MatchDirective {
    /// Create an instance from a string slice.
    //
    // This is a private method rather than a trait because we assume the
    // input string is a single-value only. This is upheld by the calling
    // function, but we cannot guarantee this to be true in the general
    // sense.
    pub(crate) fn from_str(s: &str) -> crate::Result<Option<Self>> {
        let s = s.trim();

        // We're dealing with an empty string.
        if s.is_empty() {
            return Ok(None);
        }

        match s {
            "*" => Ok(Some(MatchDirective::Wildcard)),
            s => ETag::from_str(s).map(|etag| Some(MatchDirective::ETag(etag))),
        }
    }
}

impl From<ETag> for MatchDirective {
    fn from(etag: ETag) -> Self {
        Self::ETag(etag)
    }
}

impl PartialEq<ETag> for MatchDirective {
    fn eq(&self, other: &ETag) -> bool {
        match self {
            Self::ETag(etag) => etag.eq(other),
            Self::Wildcard => false,
        }
    }
}

impl<'a> PartialEq<ETag> for &'a MatchDirective {
    fn eq(&self, other: &ETag) -> bool {
        match self {
            MatchDirective::ETag(etag) => etag.eq(other),
            MatchDirective::Wildcard => false,
        }
    }
}

impl From<MatchDirective> for HeaderValue {
    fn from(directive: MatchDirective) -> Self {
        match directive {
            MatchDirective::ETag(etag) => etag.value(),
            MatchDirective::Wildcard => unsafe {
                HeaderValue::from_bytes_unchecked("*".to_string().into_bytes())
            },
        }
    }
}
