//! HTTP Content headers.
//!
//! These headers are used for "content negotiation": the client shares information
//! about which content it prefers, and the server responds by sharing which
//! content it's chosen to share. This enables clients to receive resources with the
//! best available compression, in the preferred language, and more.

pub mod accept_encoding;
pub mod content_encoding;

mod encoding;
mod encoding_proposal;
mod content_location;

#[doc(inline)]
pub use accept_encoding::AcceptEncoding;
#[doc(inline)]
pub use content_encoding::ContentEncoding;
pub use encoding::Encoding;
pub use encoding_proposal::EncodingProposal;
pub use content_location::ContentLocation;
