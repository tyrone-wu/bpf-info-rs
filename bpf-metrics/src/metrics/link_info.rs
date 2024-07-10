//! Metrics for `bpf_link_info`.

use aya::programs::loaded_links;
use aya_obj::generated::{bpf_link_info, bpf_link_type};
use prometheus_client::{encoding::EncodeLabelSet, registry::Registry};

use crate::metric_collection::{Collector, MetricCollection};

/// Metric options for the `bpf_link_info` object.
///
/// # Example
///
/// ```no_run
/// use bpf_info::{BpfMetrics, MapMetric};
///
/// // Init metrics registry
/// let mut bpf_metrics = BpfMetrics::new();
///
/// // Select and register metrics of interest
/// let metrics = [];
/// bpf_metrics.register_link_metrics(metrics.iter());
/// ```
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum LinkMetric {
    ///
    Ping,
}

/// Label identifier for a link metric.
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub(crate) struct LinkLabels {
    /// Link type
    link_type: String,
    /// Unique ID of the link object
    id: u32,
    /// Program ID that the link object is linked to
    prog_id: u32,
}

impl LinkLabels {
    fn new(link_info: &bpf_link_info) -> Self {
        Self {
            link_type: bpf_link_type::from(link_info.type_).to_string(),
            id: link_info.id,
            prog_id: link_info.prog_id,
        }
    }
}

impl MetricCollection<LinkMetric, LinkLabels> {
    /// Init and attach sub-registry to root registry, with the selected link metrics.
    pub(crate) fn init_with_metrics<'a>(
        registry: &mut Registry,
        _metrics_iter: impl Iterator<Item = &'a LinkMetric>,
    ) -> Self {
        let _link_registry = registry.sub_registry_with_prefix("link");
        let mut _link_metrics = MetricCollection::<LinkMetric, LinkLabels>::default();

        _link_metrics
    }
}

impl Collector for MetricCollection<LinkMetric, LinkLabels> {
    fn collect_metrics(&self) {
        for link in loaded_links() {
            if let Ok(info) = link {
                let _labels = LinkLabels::new(&info);
            }
        }
    }
}
