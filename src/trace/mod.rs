//! Extract and inject [trace context](https://w3c.github.io/trace-context/) headers.
//!
//! ## Examples
//!
//! ```
//! use http_types::trace::TraceContext;
//!
//! let mut res = http_types::Response::new(200);
//!
//! res.insert_header(
//!     "traceparent",
//!     "00-0af7651916cd43dd8448eb211c80319c-00f067aa0ba902b7-01"
//! );
//!
//! let context = TraceContext::extract(&res).unwrap();
//!
//! let trace_id = u128::from_str_radix("0af7651916cd43dd8448eb211c80319c", 16);
//! let parent_id = u64::from_str_radix("00f067aa0ba902b7", 16);
//!
//! assert_eq!(context.trace_id(), trace_id.unwrap());
//! assert_eq!(context.parent_id(), parent_id.ok());
//! assert_eq!(context.sampled(), true);
//! ```

mod server_timing;
mod trace_context;

pub use server_timing::ServerTiming;
pub use trace_context::TraceContext;
