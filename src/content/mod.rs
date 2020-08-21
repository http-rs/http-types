//! HTTP Content headers.

pub mod content_encoding;

mod encoding;
mod encoding_proposal;

#[doc(inline)]
pub use content_encoding::ContentEncoding;
pub use encoding::Encoding;
pub use encoding_proposal::EncodingProposal;
