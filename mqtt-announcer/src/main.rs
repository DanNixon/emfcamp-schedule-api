mod smol_event;

use crate::smol_event::SmolEvent;
use chrono::Duration as ChronoDuration;
use clap::Parser;
use emfcamp_schedule_api::{
    announcer::{Announcer, AnnouncerPollResult, AnnouncerSettingsBuilder},
    Client as ScheduleClient,
};
use metrics::{counter, describe_counter};
use metrics_exporter_prometheus::PrometheusBuilder;
use rumqttc::{AsyncClient, LastWill, MqttOptions, QoS};
use serde::Serialize;
use std::{net::SocketAddr, time::Duration as StdDuration};
use tracing::{debug, error, info, warn};
use url::Url;

const EVENT_ANNOUNCEMENTS_METRIC: &str = "mqtt_event_announcements";

/// Announces the EMF schedule via DAPNET
#[derive(Debug, Parser)]
struct Cli {
    /// Address of schedule API to source event data from
    #[arg(
        long,
        env,
        default_value = "https://schedule.emfcamp.dan-nixon.com/schedule"
    )]
    api_url: Url,

    /// Time in seconds before the start time of an event to send the notification
    #[arg(long, env)]
    pre_event_announcement_time: i64,

    /// Hostname of the MQTT broker to connect to
    #[arg(long, env, default_value = "127.0.0.1")]
    mqtt_broker: String,

    /// Port on which to connect to the MQTT broker
    #[arg(long, env, default_value = "1883")]
    mqtt_port: u16,

    /// Client ID to use for MQTT connection
    #[arg(long, env, default_value = "emfcamp-mqtt-schedule-announcer")]
    mqtt_client_id: String,

    /// MQTT username
    #[arg(long, env)]
    mqtt_username: Option<String>,

    /// MQTT password
    #[arg(long, env)]
    mqtt_password: Option<String>,

    /// Prefix to use when building MQTT topics
    #[arg(long, env)]
    mqtt_topic_prefix: String,

    /// Address on which to run the metrics endpoint
    #[arg(long, env, default_value = "127.0.0.1:9090")]
    observability_address: SocketAddr,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    tracing_subscriber::fmt::init();

    // Set up metrics server
    let builder = PrometheusBuilder::new();
    builder
        .with_http_listener(cli.observability_address)
        .install()?;

    describe_counter!(
        EVENT_ANNOUNCEMENTS_METRIC,
        "Number of announcements sent via MQTT"
    );

    // Setup schedule API client
    let schedule_client = ScheduleClient::new(cli.api_url);

    let event_start_offset = -ChronoDuration::try_seconds(cli.pre_event_announcement_time)
        .ok_or_else(|| anyhow::anyhow!("Invalid pre event announcement time"))?;
    info!("Event start offset: {:?}", event_start_offset);

    let mut announcer = Announcer::new(
        AnnouncerSettingsBuilder::default()
            .event_start_offset(event_start_offset)
            .build()?,
        schedule_client,
    )
    .await?;

    // Configure MQTT broker connection
    let online_topic = format!("{}/online", cli.mqtt_topic_prefix);

    let mqtt_options = {
        let mut options = MqttOptions::new(cli.mqtt_client_id, cli.mqtt_broker, cli.mqtt_port);
        options.set_keep_alive(StdDuration::from_secs(5));

        options.set_last_will(LastWill::new(
            &online_topic,
            "false",
            QoS::AtLeastOnce,
            true,
        ));

        if cli.mqtt_username.is_some() && cli.mqtt_password.is_some() {
            info!("Using supplied MQTT credentials");
            options.set_credentials(cli.mqtt_username.unwrap(), cli.mqtt_password.unwrap());
        } else {
            info!("Not attempting to authenticate MQTT connection");
        }

        options
    };

    let (mqtt_client, mut mqtt_eventloop) = AsyncClient::new(mqtt_options, 10);

    // Send alive message
    if let Err(e) = mqtt_client
        .publish(online_topic, QoS::AtLeastOnce, true, "true")
        .await
    {
        warn!("Failed to send alive MQTT message: {e}");
    }

    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                return Ok(());
            }
            msg = announcer.poll() => {
                handle_announcer_event(&mqtt_client, &cli.mqtt_topic_prefix, msg).await;
            }
            event = mqtt_eventloop.poll() => {
                match event {
                    Ok(event) => {
                        debug!("MQTT event: {:?}", event);
                    }
                    Err(e) => {
                        warn!("MQTT error: {:?}", e);
                        tokio::time::sleep(StdDuration::from_secs(1)).await;
                    }
                }
            }
        }
    }
}

async fn handle_announcer_event(
    mqtt_client: &AsyncClient,
    topic_prefix: &str,
    msg: emfcamp_schedule_api::Result<AnnouncerPollResult>,
) {
    match msg {
        Ok(AnnouncerPollResult::Event(event)) => {
            debug!("Event: {:?}", event);

            // Send full event JSON
            send_event_data(mqtt_client, topic_prefix, "full", &event).await;

            // Send smol event JSON
            let smol_event: SmolEvent = event.into();
            send_event_data(mqtt_client, topic_prefix, "smol", &smol_event).await;
        }
        Err(e) => {
            warn!("{e}");
        }
        _ => {}
    }
}

async fn send_event_data<T: Serialize>(
    mqtt_client: &AsyncClient,
    topic_prefix: &str,
    size_id: &'static str,
    event: &T,
) {
    info!("Sending {size_id} event");

    match serde_json::to_string(event) {
        Ok(event_json) => {
            match mqtt_client
                .publish(
                    format!("{topic_prefix}/{size_id}"),
                    QoS::AtLeastOnce,
                    false,
                    event_json,
                )
                .await
            {
                Ok(_) => {
                    info!("MQTT message sent");
                    counter!(EVENT_ANNOUNCEMENTS_METRIC, "size" => size_id, "result" => "ok")
                        .increment(1);
                }
                Err(e) => {
                    error!("Failed to send MQTT message: {e}");
                    counter!(EVENT_ANNOUNCEMENTS_METRIC, "size" => size_id, "result" => "error")
                        .increment(1);
                }
            }
        }
        Err(e) => error!("Failed to serialize event (this should not happen!): {e}"),
    }
}
