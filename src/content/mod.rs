//! HTTP Content headers.

pub mod content_encoding;

mod encoding_directive;

#[doc(inline)]
pub use content_encoding::ContentEncoding;
pub use encoding_directive::EncodingDirective;
