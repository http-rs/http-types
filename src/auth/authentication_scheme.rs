use std::fmt::{self, Display};
use std::str::FromStr;

use crate::bail;

/// HTTP Mutual Authentication Algorithms
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum AuthenticationScheme {
    /// Basic auth
    Basic,
    /// Bearer auth
    Bearer,
    /// Digest auth
    Digest,
    /// HOBA
    Hoba,
    /// Mutual auth
    Mutual,
    /// Negotiate auth
    Negotiate,
    /// Oauth
    OAuth,
    /// SCRAM SHA1 auth
    ScramSha1,
    /// SCRAM SHA256 auth
    ScramSha256,
    /// Vapid auth
    Vapid,
}

impl Display for AuthenticationScheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Basic => write!(f, "Basic"),
            Self::Bearer => write!(f, "Bearer"),
            Self::Digest => write!(f, "Digest"),
            Self::Hoba => write!(f, "HOBA"),
            Self::Mutual => write!(f, "Mutual"),
            Self::Negotiate => write!(f, "Negotiate"),
            Self::OAuth => write!(f, "OAuth"),
            Self::ScramSha1 => write!(f, "SCRAM-SHA-1"),
            Self::ScramSha256 => write!(f, "SCRAM-SHA-256"),
            Self::Vapid => write!(f, "vapid"),
        }
    }
}

impl FromStr for AuthenticationScheme {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Basic" => Ok(Self::Basic),
            "Bearer" => Ok(Self::Bearer),
            "Digest" => Ok(Self::Digest),
            "HOBA" => Ok(Self::Hoba),
            "Mutual" => Ok(Self::Mutual),
            "Negotiate" => Ok(Self::Negotiate),
            "OAuth" => Ok(Self::OAuth),
            "SCRAM-SHA-1" => Ok(Self::ScramSha1),
            "SCRAM-SHA-256" => Ok(Self::ScramSha256),
            "vapid" => Ok(Self::Vapid),
            s => bail!("`{}` is not a recognized authentication scheme", s),
        }
    }
}
