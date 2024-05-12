/// This is intended as a long running test case for the announcer.
///
/// It will use the 2022 schedule with a fake epoch via the adapter to simulate announcing the full
/// 2022 schedule in real time.
///
/// There is some automatic checking that things are happening at the correct times and sufficient
/// console output to manually verify this.
///
/// You will need the adapter running locally and pointed towards the 2022 schedule API first:
/// ```shell
/// cargo run --bin emfcamp-schedule-api-adapter --release -- --upstream-api-url https://www.emfcamp.org/schedule/2022.json
/// ```
///
/// This example can then be run, ideally with lower level logging set:
/// ```shell
/// RUST_LOG=info cargo run --example announcer_long_test --release
/// ```
use anyhow::Result;
use chrono::{DateTime, FixedOffset, SecondsFormat, Utc};
use emfcamp_schedule_api::{
    announcer::{Announcer, AnnouncerPollResult, AnnouncerSettingsBuilder},
    Client,
};
use tracing::{error, info};
use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let start_time: DateTime<FixedOffset> = Utc::now().into();
    let start_point = tokio::time::Instant::now();

    let epoch = start_time + chrono::Duration::try_minutes(2).unwrap();

    let url = {
        let mut url = Url::parse("http://localhost:8000/schedule")?;

        url.set_query(Some(
            &url::form_urlencoded::Serializer::new(String::default())
                .append_pair(
                    "fake_epoch",
                    &epoch.to_rfc3339_opts(SecondsFormat::Secs, false),
                )
                .finish(),
        ));

        info!("Schedule API URL: {url}");
        url
    };

    let client = Client::new(url);

    let event_start_offset = chrono::Duration::try_minutes(-1).unwrap();
    let settings = AnnouncerSettingsBuilder::default()
        .schedule_refresh(tokio::time::Duration::from_secs(60))
        .event_start_offset(event_start_offset)
        .build()?;

    // Output the expected events and the times they should be announced
    {
        let schedule = client.get_schedule().await?;

        println!("---BEGIN EXPECTED---");
        for event in schedule.events {
            let expect_announcement_in = event.start + event_start_offset - start_time;
            println!(
                "{}s: {} {} \"{}\"",
                expect_announcement_in.num_seconds(),
                event.start,
                event.id,
                event.title
            );
        }
        println!("---END EXPECTED---");
    }

    let mut announcer = Announcer::new(settings, client).await?;

    println!("---BEGIN ACTUAL---");
    loop {
        let event = announcer.poll().await;
        info!("poll event: {:?}", event);

        match event {
            Ok(AnnouncerPollResult::Event(event)) => {
                // Output the event and when it was announced
                let now = tokio::time::Instant::now();
                let elapsed = (now - start_point).as_secs();
                println!(
                    "{elapsed}s: {} {} \"{}\"",
                    event.start, event.id, event.title
                );

                // Check that it was announced at roughly the correct time
                let expected_announcement_in =
                    (event.start + event_start_offset - start_time).num_seconds();
                let diff = (expected_announcement_in - elapsed as i64).abs();

                if diff <= 5 {
                    info!("Announced at about the correct time (diff={diff})");
                } else {
                    error!(
                        "Announced at the incorrect time ({elapsed} vs {expected_announcement_in})"
                    );
                }
            }
            Err(e) => error!("{e}"),
            _ => {}
        };
    }
}
