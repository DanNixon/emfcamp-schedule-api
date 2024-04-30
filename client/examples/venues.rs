use anyhow::Result;
use emfcamp_schedule_api::Client;
use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
    let url = Url::parse("https://www.emfcamp.org/schedule/2024.json")?;

    let client = Client::new(url);

    let schedule = client.get_schedule().await?;

    let venues = schedule.venues();

    println!("Found {} venues:", venues.len());
    for venue in venues {
        println!("- {}", venue);
    }

    Ok(())
}
