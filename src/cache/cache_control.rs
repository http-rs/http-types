/// An HTTP `Cache-Control` directive.
#[derive(Debug)]
pub enum CacheDirective {
    Immutable,
    MaxAge(Duration),
    MaxStale(Option<Duration>),
    MinFresh(Duration),
    MustRevalidate,
    NoCache,
    NoStore,
    NoTransform,
    OnlyIfCached,
    Private,
    ProxyRevalidate,
    Public,
    SMaxAge(Duration),
    StaleIfError(Duration),
    StaleWhileRevalidate(Duration),
}

impl CacheDirective {
    /// Check whether this directive is valid in an HTTP request.
    pub fn is_req(&self) -> bool {
        use Self::*;
        match self {
            MaxAge(_) | MaxStale(_) | MinFresh(_) | NoCache | NoStore | NoTransform
            | OnlyIfCached => true,
            _ => false,
        }
    }

    /// Check whether this directive is valid in an HTTP response.
    pub fn is_res(&self) -> bool {
        use Self::*;
        match self {
            MustRevalidate | NoCache | NoStore | NoTransform | Public | Private
            | ProxyRevalidate | MaxAge(_) | SMaxAge(_) => true,
            _ => false,
        }
    }
}
