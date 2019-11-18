/// The version of the HTTP protocol in use
#[derive(Copy, Clone, Debug)]
pub enum HttpVersion {
    /// HTTP 1.0
    HTTP1_0,

    /// HTTP 1.1
    HTTP1_1,

    /// HTTP 2.0
    HTTP2_0,

    /// HTTP 3.0
    HTTP3_0,
}
