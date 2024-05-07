pub(crate) mod metrics;
mod queries;

use crate::queries::now_and_next::now_and_next;
use crate::queries::schedule::schedule;
use crate::queries::venues::venues;
use anyhow::Result;
use axum::{routing::get, Router};
use clap::Parser;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{info, trace};
use url::Url;

#[derive(Parser)]
#[clap(version, about)]
struct Cli {
    #[clap(
        long,
        env,
        default_value = "https://www.emfcamp.org/schedule/2024.json"
    )]
    upstream_api_url: Url,

    #[clap(long, env, default_value = "127.0.0.1:8000")]
    api_address: SocketAddr,

    #[clap(long, env, default_value = "127.0.0.1:9090")]
    observability_address: SocketAddr,
}

#[derive(Clone)]
struct State {
    client: emfcamp_schedule_api::Client,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    tracing_subscriber::fmt::init();

    crate::metrics::init(args.observability_address)?;

    trace!("Creating client");
    let client = emfcamp_schedule_api::Client::new(args.upstream_api_url);

    let state = State { client };

    let app = Router::new()
        .route("/schedule", get(schedule))
        .route("/now-and-next", get(now_and_next))
        .route("/venues", get(venues))
        .with_state(state);

    info!("API shim running at {}", args.api_address);
    let listener = TcpListener::bind(&args.api_address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
