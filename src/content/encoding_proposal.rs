use crate::content::Encoding;
use crate::ensure;
use crate::headers::HeaderValue;

use std::cmp::{Ordering, PartialEq};

/// A proposed `Encoding` in `AcceptEncoding`.
#[derive(Debug, Clone, PartialEq)]
pub struct EncodingProposal {
    /// The proposed encoding.
    encoding: Encoding,

    /// The weight of the proposal.
    ///
    /// This is a number between 0.0 and 1.0, and is max 3 decimal points.
    weight: Option<f32>,
}

impl EncodingProposal {
    /// Create a new instance of `EncodingProposal`.
    pub fn new(encoding: impl Into<Encoding>, weight: Option<f32>) -> crate::Result<Self> {
        if let Some(weight) = weight {
            ensure!(
                weight < 0.0 || weight > 1.0,
                "EncodingProposal should have a weight between 0.0 and 1.0"
            )
        }

        Ok(Self {
            encoding: encoding.into(),
            weight,
        })
    }

    /// Get the proposed encoding.
    pub fn encoding(&self) -> &Encoding {
        &self.encoding
    }

    /// Get the weight of the proposal.
    pub fn weight(&self) -> Option<f32> {
        self.weight
    }
}

impl From<Encoding> for EncodingProposal {
    fn from(encoding: Encoding) -> Self {
        Self {
            encoding,
            weight: None,
        }
    }
}

impl PartialEq<Encoding> for EncodingProposal {
    fn eq(&self, other: &Encoding) -> bool {
        self.encoding == *other
    }
}

// NOTE: Firefox populates Accept-Encoding as `gzip, deflate, br`. This means
// when parsing encodings we should choose the last value in the list under
// equal weights. This impl doesn't know which value was passed later, so that
// behavior needs to be handled separately.
//
// NOTE: This comparison does not include a notion of `*` (any value is valid).
// that needs to be handled separately.
impl PartialOrd for EncodingProposal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.weight, other.weight) {
            (Some(left), Some(right)) => left.partial_cmp(&right),
            (Some(_), None) => Some(Ordering::Greater),
            (None, Some(_)) => Some(Ordering::Less),
            (None, None) => None,
        }
    }
}

impl From<EncodingProposal> for HeaderValue {
    fn from(entry: EncodingProposal) -> HeaderValue {
        let s = match entry.weight {
            Some(weight) => format!("{};q={:.3}", entry.encoding, weight),
            None => entry.encoding.to_string(),
        };
        unsafe { HeaderValue::from_bytes_unchecked(s.into_bytes()) }
    }
}
