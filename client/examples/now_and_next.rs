use anyhow::Result;
use chrono::Local;
use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
    let url = Url::parse("https://www.emfcamp.org/schedule/2024.json")?;

    let client = emfcamp_schedule_api::Client::new(url);

    let schedule = client.get_schedule().await?;

    let now = Local::now();
    let now_and_next = schedule.now_and_next(now.into());

    println!("{:#?}", now_and_next);

    Ok(())
}
