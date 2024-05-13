use super::*;

#[tokio::test]
async fn t14_notifications_start_at_correct_point_in_time_on_fresh_start() {
    let mut dummy_server = DummyScheduleServer::new(8014).await;

    let now = Utc::now();

    dummy_server.set_events(vec![
        Event::dummy(0, (now + ChronoDuration::try_seconds(-2).unwrap()).into()),
        Event::dummy(1, (now + ChronoDuration::try_seconds(0).unwrap()).into()),
        Event::dummy(2, (now + ChronoDuration::try_seconds(2).unwrap()).into()),
        Event::dummy(3, (now + ChronoDuration::try_seconds(4).unwrap()).into()),
    ]);

    let client = Client::new(dummy_server.url());

    let now_i = Instant::now();
    let mut announcer = Announcer::new(
        AnnouncerSettingsBuilder::default()
            .schedule_refresh(Duration::from_secs(600))
            .event_start_offset(ChronoDuration::zero())
            .build()
            .unwrap(),
        client,
    )
    .await
    .unwrap();

    crate::assert_future_in!(
        announcer.poll(),
        now_i + Duration::from_secs(2),
        AnnouncerPollResult::Event(dummy_server.event(2))
    );

    crate::assert_future_in!(
        announcer.poll(),
        now_i + Duration::from_secs(4),
        AnnouncerPollResult::Event(dummy_server.event(3))
    );

    dummy_server.stop().await;
}
