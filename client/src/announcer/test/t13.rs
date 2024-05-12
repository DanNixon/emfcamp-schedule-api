use super::*;

#[tokio::test]
async fn t13_basic_event_notification_with_offset() {
    let mut dummy_server = DummyScheduleServer::new(8013).await;

    let now = Utc::now();

    dummy_server.set_events(vec![
        Event::dummy(0, (now + ChronoDuration::try_seconds(1).unwrap()).into()),
        Event::dummy(1, (now + ChronoDuration::try_seconds(2).unwrap()).into()),
        Event::dummy(2, (now + ChronoDuration::try_seconds(3).unwrap()).into()),
    ]);

    let client = Client::new(dummy_server.url());

    let now_i = Instant::now();
    let mut announcer = Announcer::new(
        AnnouncerSettingsBuilder::default()
            .schedule_refresh(Duration::from_secs(600))
            .event_start_offset(ChronoDuration::try_seconds(-1).unwrap())
            .build()
            .unwrap(),
        client,
    )
    .await
    .unwrap();

    crate::assert_future_in!(
        announcer.poll(),
        now_i + Duration::from_secs(0),
        AnnouncerPollResult::Event(dummy_server.event(0))
    );

    crate::assert_future_in!(
        announcer.poll(),
        now_i + Duration::from_secs(1),
        AnnouncerPollResult::Event(dummy_server.event(1))
    );

    crate::assert_future_in!(
        announcer.poll(),
        now_i + Duration::from_secs(2),
        AnnouncerPollResult::Event(dummy_server.event(2))
    );

    dummy_server.stop().await;
}
