use crate::ensure;
use crate::headers::HeaderValue;
use crate::language::LanguageRange;
use crate::utils::parse_weight;

use std::cmp::{Ordering, PartialEq};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

/// A proposed `LanguageRange` in `AcceptLanguage`.
#[derive(Debug, Clone, PartialEq)]
pub struct LanguageProposal {
    /// The proposed language.
    pub(crate) language: LanguageRange,

    /// The weight of the proposal.
    ///
    /// This is a number between 0.0 and 1.0, and is max 3 decimal points.
    weight: Option<f32>,
}

impl LanguageProposal {
    /// Create a new instance of `LanguageProposal`.
    pub fn new(language: impl Into<LanguageRange>, weight: Option<f32>) -> crate::Result<Self> {
        if let Some(weight) = weight {
            ensure!(
                weight.is_sign_positive() && weight <= 1.0,
                "LanguageProposal should have a weight between 0.0 and 1.0"
            )
        }

        Ok(Self {
            language: language.into(),
            weight,
        })
    }

    /// Get the proposed language.
    pub fn language_range(&self) -> &LanguageRange {
        &self.language
    }

    /// Get the weight of the proposal.
    pub fn weight(&self) -> Option<f32> {
        self.weight
    }

    pub(crate) fn from_str(s: &str) -> crate::Result<Self> {
        let mut parts = s.split(';');
        let language = LanguageRange::from_str(parts.next().unwrap())?;
        let weight = parts.next().map(parse_weight).transpose()?;
        Ok(Self::new(language, weight)?)
    }
}

impl From<LanguageRange> for LanguageProposal {
    fn from(language: LanguageRange) -> Self {
        Self {
            language,
            weight: None,
        }
    }
}

impl PartialEq<LanguageRange> for LanguageProposal {
    fn eq(&self, other: &LanguageRange) -> bool {
        self.language == *other
    }
}

impl PartialEq<LanguageRange> for &LanguageProposal {
    fn eq(&self, other: &LanguageRange) -> bool {
        self.language == *other
    }
}

impl Deref for LanguageProposal {
    type Target = LanguageRange;
    fn deref(&self) -> &Self::Target {
        &self.language
    }
}

impl DerefMut for LanguageProposal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.language
    }
}

impl PartialOrd for LanguageProposal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.weight, other.weight) {
            (Some(left), Some(right)) => left.partial_cmp(&right),
            (Some(_), None) => Some(Ordering::Greater),
            (None, Some(_)) => Some(Ordering::Less),
            (None, None) => None,
        }
    }
}

impl From<LanguageProposal> for HeaderValue {
    fn from(entry: LanguageProposal) -> HeaderValue {
        let s = match entry.weight {
            Some(weight) => format!("{};q={:.3}", entry.language, weight),
            None => entry.language.to_string(),
        };
        unsafe { HeaderValue::from_bytes_unchecked(s.into_bytes()) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke() {
        let _ = LanguageProposal::new("en", Some(1.0)).unwrap();
    }

    #[test]
    fn error_code_500() {
        let err = LanguageProposal::new("en", Some(1.1)).unwrap_err();
        assert_eq!(err.status(), 500);
    }
}
