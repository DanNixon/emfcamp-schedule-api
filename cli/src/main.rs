#[tokio::main]
async fn main() {
    let c = emfcamp_schedule_api::Client::new(
        url::Url::parse("https://www.emfcamp.org/schedule/2022.json").unwrap(),
    );
    let sched = c.get_schedule().await;
    println!("{:#?}", sched);
}
