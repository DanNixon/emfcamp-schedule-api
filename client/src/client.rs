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

    pub async fn get_schedule(&self) -> Schedule {
        let events = reqwest::get(self.url.clone())
            .await
            .unwrap()
            .json::<Vec<Event>>()
            .await
            .unwrap();

        Schedule { events }
    }
}
