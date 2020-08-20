use crate::headers::HeaderValue;

/// Available compression algorithms.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum EncodingDirective {
    /// The Gzip encoding.
    Gzip,
    /// The Deflate encoding.
    Deflate,
    /// The Brotli encoding.
    Brotli,
    /// The Zstd encoding.
    Zstd,
    /// No encoding.
    Identity,
}

impl EncodingDirective {
    /// Parses a given string into its corresponding encoding.
    pub(crate) fn from_str(s: &str) -> Option<EncodingDirective> {
        let s = s.trim();

        // We're dealing with an empty string.
        if s.is_empty() {
            return None;
        }

        match s {
            "gzip" => Some(EncodingDirective::Gzip),
            "deflate" => Some(EncodingDirective::Deflate),
            "br" => Some(EncodingDirective::Brotli),
            "zstd" => Some(EncodingDirective::Zstd),
            "identity" => Some(EncodingDirective::Identity),
            _ => None,
        }
    }
}

impl From<EncodingDirective> for HeaderValue {
    fn from(directive: EncodingDirective) -> Self {
        let h = |s: &str| unsafe { HeaderValue::from_bytes_unchecked(s.to_string().into_bytes()) };

        match directive {
            EncodingDirective::Gzip => h("gzip"),
            EncodingDirective::Deflate => h("deflate"),
            EncodingDirective::Brotli => h("br"),
            EncodingDirective::Zstd => h("zstd"),
            EncodingDirective::Identity => h("identity"),
        }
    }
}
