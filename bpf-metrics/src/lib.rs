#![deny(dead_code)]
#![deny(missing_docs)]
#![deny(unused)]

//! Collects and publishes eBPF metadata for programs, maps, and links on the host system, exposing
//! metrics using the OpenMetrics exposition format.
//!
//! ```no_run
//! todo!()
//! ```

mod bpf_stats;

#[cfg(feature = "bpf-stats")]
pub use bpf_stats::*;
