use crate::schedule::{event::Event, Schedule};
use url::Url;

#[derive(Debug, Clone)]
pub struct Client {
    url: Url,
}

impl Client {
    pub fn new(url: Url) -> Self {
        Self { url }
    }

    pub async fn get_schedule(&self) -> crate::Result<Schedule> {
        let events = reqwest::get(self.url.clone())
            .await?
            .json::<Vec<Event>>()
            .await?;

        Ok(Schedule { events })
    }
}
