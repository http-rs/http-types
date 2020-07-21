//! HTTP timings and traces.
//!
//! This module implements parsers and serializers for timing-related headers.
//! These headers enable tracing and timing requests, and help answer the
//! question of: _"Where is my program spending its time?"_
//!
//! # Specifications
//!
//! - [W3C Trace-Context headers](https://w3c.github.io/trace-context/)
//! - [W3C Server-Timing headers](https://w3c.github.io/server-timing/#the-server-timing-header-field)

mod server_timing;
mod trace_context;

pub use server_timing::{ServerTiming, TimingEntry};
pub use trace_context::TraceContext;
