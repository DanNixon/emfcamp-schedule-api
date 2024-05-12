use super::*;

#[tokio::test]
async fn t08_future_changes_take_effect() {
    let mut dummy_server = DummyScheduleServer::new(8008).await;

    let now = Utc::now();

    let events = set_and_patch_dummy_events(
        &mut dummy_server,
        vec![
            Event::dummy(0, (now + ChronoDuration::try_seconds(1).unwrap()).into()),
            Event::dummy(1, (now + ChronoDuration::try_seconds(3).unwrap()).into()),
            Event::dummy(2, (now + ChronoDuration::try_seconds(7).unwrap()).into()),
        ],
    );

    let client = Client::new(dummy_server.url());

    let now_i = Instant::now();
    let mut announcer = Announcer::new(
        AnnouncerSettingsBuilder::default()
            .schedule_refresh(Duration::from_secs(2))
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
        AnnouncerPollResult::ScheduleRefreshed(AnnouncerScheduleChanges::NoChanges)
    );

    crate::assert_future_in!(
        announcer.poll(),
        now_i + Duration::from_secs(3),
        AnnouncerPollResult::Event(events[1].clone())
    );

    crate::assert_future_in!(
        announcer.poll(),
        now_i + Duration::from_secs(4),
        AnnouncerPollResult::ScheduleRefreshed(AnnouncerScheduleChanges::NoChanges)
    );

    let events = set_and_patch_dummy_events(
        &mut dummy_server,
        vec![
            Event::dummy(0, (now + ChronoDuration::try_seconds(1).unwrap()).into()),
            Event::dummy(1, (now + ChronoDuration::try_seconds(3).unwrap()).into()),
            Event::dummy(2, (now + ChronoDuration::try_seconds(9).unwrap()).into()),
        ],
    );

    crate::assert_future_in!(
        announcer.poll(),
        now_i + Duration::from_secs(6),
        AnnouncerPollResult::ScheduleRefreshed(AnnouncerScheduleChanges::Changes)
    );

    crate::assert_future_in!(
        announcer.poll(),
        now_i + Duration::from_secs(8),
        AnnouncerPollResult::ScheduleRefreshed(AnnouncerScheduleChanges::NoChanges)
    );

    crate::assert_future_in!(
        announcer.poll(),
        now_i + Duration::from_secs(9),
        AnnouncerPollResult::Event(events[2].clone())
    );

    crate::assert_future_in!(
        announcer.poll(),
        now_i + Duration::from_secs(10),
        AnnouncerPollResult::ScheduleRefreshed(AnnouncerScheduleChanges::NoChanges)
    );

    dummy_server.stop().await;
}
