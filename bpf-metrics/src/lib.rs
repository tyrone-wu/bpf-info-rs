#![deny(dead_code)]
#![deny(missing_docs)]
#![deny(unused)]

//! Collects and publishes eBPF metadata for programs, maps, and links on the host system, exposing
//! metrics using the OpenMetrics exposition format.
//!
//! ```no_run
//! use bpf_info::{BpfMetrics, LinkMetric, MapMetric, ProgMetric};
//!
//! // Init registry
//! let mut bpf_metrics = BpfMetrics::new();
//!
//! // Define and register metrics of interest
//! let prog_metrics = [ProgMetric::Uptime, ProgMetric::MemoryLocked];
//! bpf_metrics.register_prog_metrics(prog_metrics.iter());
//!
//! let map_metrics = [MapMetric::MaxEntries];
//! bpf_metrics.register_map_metrics(map_metrics.iter());
//!
//! let link_metrics = [];
//! bpf_metrics.register_link_metrics(link_metrics.iter());
//!
//! // Collect bpf metrics from the host
//! bpf_metrics.collect_metrics();
//!
//! // Export metrics in OpenMetrics text format
//! let mut buffer = String::new();
//! bpf_metrics.export_metrics(&mut buffer);
//! ```

mod bpf_metrics;
mod bpf_stats;
pub(crate) mod metric_collection;
mod metrics;

#[cfg(feature = "metrics")]
pub use bpf_metrics::BpfMetrics;
#[cfg(feature = "bpf-stats")]
pub use bpf_stats::*;
#[cfg(feature = "metrics")]
pub use metrics::link_info::LinkMetric;
#[cfg(feature = "metrics")]
pub use metrics::map_info::MapMetric;
#[cfg(feature = "metrics")]
pub use metrics::prog_info::ProgMetric;
