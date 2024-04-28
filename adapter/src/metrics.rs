use std::net::SocketAddr;
use tracing::info;

use metrics::describe_counter;
use metrics_exporter_prometheus::{BuildError, PrometheusBuilder};

pub(crate) static REQUESTS: &str = "emf_schedule_adapter_requests";
pub(crate) static ENDPOINT_LABEL: &str = "endpoint";

pub(super) fn init(address: SocketAddr) -> Result<(), BuildError> {
    info!("Starting observability server on {address}");

    let result = PrometheusBuilder::new()
        .with_http_listener(address)
        .install();

    describe_counter!(REQUESTS, "Number of requests made to each API endpoint");

    result
}
