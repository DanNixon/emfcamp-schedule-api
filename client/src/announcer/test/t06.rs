use super::*;

#[tokio::test]
async fn t06_basic_event_notification_unsorted() {
    let mut dummy_server = DummyScheduleServer::new(8006).await;

    let now = Utc::now();
    let mut events = vec![
        Event::dummy(0, (now + ChronoDuration::try_seconds(1).unwrap()).into()),
        Event::dummy(1, (now + ChronoDuration::try_seconds(2).unwrap()).into()),
        Event::dummy(2, (now + ChronoDuration::try_seconds(3).unwrap()).into()),
    ];

    dummy_server.set_events(vec![
        events[1].clone(),
        events[0].clone(),
        events[2].clone(),
    ]);

    fixup_events_for_test_comparison(&mut events);

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
        now_i + Duration::from_secs(1),
        AnnouncerPollResult::Event(events[0].clone())
    );

    crate::assert_future_in!(
        announcer.poll(),
        now_i + Duration::from_secs(2),
        AnnouncerPollResult::Event(events[1].clone())
    );

    crate::assert_future_in!(
        announcer.poll(),
        now_i + Duration::from_secs(3),
        AnnouncerPollResult::Event(events[2].clone())
    );

    dummy_server.stop().await;
}
