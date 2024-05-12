use super::*;

#[tokio::test]
async fn t02_schedule_is_refreshed_on_requested_schedule() {
    let mut dummy_server = DummyScheduleServer::new(8002).await;

    let now = Utc::now();

    set_and_patch_dummy_events(
        &mut dummy_server,
        vec![Event::dummy(
            0,
            (now + ChronoDuration::try_minutes(1).unwrap()).into(),
        )],
    );

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

    crate::assert_future_in!(
        announcer.poll(),
        now_i + Duration::from_secs(3),
        AnnouncerPollResult::ScheduleRefreshed(AnnouncerScheduleChanges::NoChanges)
    );

    dummy_server.stop().await;
}
