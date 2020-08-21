use crate::headers::HeaderValue;

/// Available compression algorithms.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Encoding {
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

impl Encoding {
    /// Parses a given string into its corresponding encoding.
    pub(crate) fn from_str(s: &str) -> Option<Encoding> {
        let s = s.trim();

        // We're dealing with an empty string.
        if s.is_empty() {
            return None;
        }

        match s {
            "gzip" => Some(Encoding::Gzip),
            "deflate" => Some(Encoding::Deflate),
            "br" => Some(Encoding::Brotli),
            "zstd" => Some(Encoding::Zstd),
            "identity" => Some(Encoding::Identity),
            _ => None,
        }
    }
}

impl From<Encoding> for HeaderValue {
    fn from(directive: Encoding) -> Self {
        let h = |s: &str| unsafe { HeaderValue::from_bytes_unchecked(s.to_string().into_bytes()) };

        match directive {
            Encoding::Gzip => h("gzip"),
            Encoding::Deflate => h("deflate"),
            Encoding::Brotli => h("br"),
            Encoding::Zstd => h("zstd"),
            Encoding::Identity => h("identity"),
        }
    }
}
