use std::time::Duration;
/// An HTTP `Cache-Control` directive.
#[derive(Debug)]
pub enum CacheDirective {
    /// The response body will not change over time.
    Immutable,
    /// The maximum amount of time a resource is considered fresh.
    MaxAge(Duration),
    /// Indicates the client will accept a stale response.
    MaxStale(Option<Duration>),
    /// A response that will still be fresh for at least the specified duration.
    MinFresh(Duration),
    /// Once a response is stale, a fresh response must be retrieved.
    MustRevalidate,
    /// The response may be cached, but must always be revalidated before being used.
    NoCache,
    /// The response may not be cached.
    NoStore,
    /// An intermediate cache or proxy cannot edit the response body,
    /// Content-Encoding, Content-Range, or Content-Type.
    NoTransform,
    /// Do not use the network for a response.
    OnlyIfCached,
    /// The response may be stored only by a browser's cache, even if the
    /// response is normally non-cacheable
    Private,
    /// Like must-revalidate, but only for shared caches (e.g., proxies).
    ProxyRevalidate,
    /// The response may be stored by any cache, even if the response is normally
    /// non-cacheable.
    Public,
    /// Overrides max-age or the Expires header, but only for shared caches.
    SMaxAge(Duration),
    /// The client will accept a stale response if retrieving a fresh one fails.
    StaleIfError(Duration),
    /// Indicates the client will accept a stale response, while asynchronously
    /// checking in the background for a fresh one.
    StaleWhileRevalidate(Duration),
}

impl CacheDirective {
    /// Check whether this directive is valid in an HTTP request.
    pub fn is_req(&self) -> bool {
        use CacheDirective::*;
        match self {
            MaxAge(_) | MaxStale(_) | MinFresh(_) | NoCache | NoStore | NoTransform
            | OnlyIfCached => true,
            _ => false,
        }
    }

    /// Check whether this directive is valid in an HTTP response.
    pub fn is_res(&self) -> bool {
        use CacheDirective::*;
        match self {
            MustRevalidate
            | NoCache
            | NoStore
            | NoTransform
            | Public
            | Private
            | ProxyRevalidate
            | MaxAge(_)
            | SMaxAge(_)
            | StaleIfError(_)
            | StaleWhileRevalidate(_) => true,
            _ => false,
        }
    }
}
