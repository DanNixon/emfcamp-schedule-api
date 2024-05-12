use super::*;

#[tokio::test]
async fn t03_changes_to_the_schedule_are_noticed() {
    let mut dummy_server = DummyScheduleServer::new(8003).await;

    let now = Utc::now();
    dummy_server.set_events(vec![Event::dummy(
        0,
        (now + ChronoDuration::try_minutes(1).unwrap()).into(),
    )]);

    let client = Client::new(dummy_server.url());

    let now_i = Instant::now();
    let mut announcer = Announcer::new(
        AnnouncerSettingsBuilder::default()
            .schedule_refresh(Duration::from_secs(3))
            .build()
            .unwrap(),
        client,
    )
    .await
    .unwrap();

    dummy_server.set_events(vec![Event::dummy(
        1,
        (now + ChronoDuration::try_minutes(1).unwrap()).into(),
    )]);

    crate::assert_future_in!(
        announcer.poll(),
        now_i + Duration::from_secs(3),
        AnnouncerPollResult::ScheduleRefreshed(AnnouncerScheduleChanges::Changes)
    );

    dummy_server.stop().await;
}
