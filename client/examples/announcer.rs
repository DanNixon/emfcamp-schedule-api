use anyhow::Result;
use emfcamp_schedule_api::{
    announcer::{Announcer, AnnouncerSettingsBuilder},
    Client,
};
use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let url = Url::parse("https://www.emfcamp.org/schedule/2024.json")?;

    let client = Client::new(url);

    let settings = AnnouncerSettingsBuilder::default()
        .schedule_refresh(tokio::time::Duration::from_secs(5))
        .build()?;

    let mut announcer = Announcer::new(settings, client).await?;

    loop {
        let event = announcer.poll().await;
        println!("{:?}", event);
    }
}
