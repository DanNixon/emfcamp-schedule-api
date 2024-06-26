use std::net::SocketAddr;
use tracing::info;

use metrics::describe_counter;
use metrics_exporter_prometheus::{BuildError, PrometheusBuilder};

pub(crate) static REQUESTS: &str = "emf_schedule_adapter_requests_total";
pub(crate) static ENDPOINT_LABEL: &str = "endpoint";

pub(crate) static UPSTREAM_API_FAILURES: &str = "emf_schedule_adapter_upstream_api_failures_total";

pub(super) fn init(address: SocketAddr) -> Result<(), BuildError> {
    info!("Starting observability server on {address}");

    let result = PrometheusBuilder::new()
        .with_http_listener(address)
        .install();

    describe_counter!(REQUESTS, "Number of requests made to each API endpoint");

    describe_counter!(
        UPSTREAM_API_FAILURES,
        "Number of requests to the upstream schedule API that have failed"
    );

    result
}
