//! HTTP Content headers.

pub mod content_encoding;

mod encoding;

#[doc(inline)]
pub use content_encoding::ContentEncoding;
pub use encoding::Encoding;
// EncodingProposal
