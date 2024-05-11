use super::*;
use crate::testing::DummyScheduleServer;
use serde_json::Value;
use std::collections::HashMap;
use tokio::time::{Duration, Instant};

fn fixup_events_for_test_comparison(events: &mut [Event]) {
    for event in events {
        event.extra = HashMap::from([("type".to_string(), Value::String("talk".to_string()))]);
    }
}

#[tokio::test]
async fn t2_schedule_is_refreshed_on_requested_schedule() {
    let mut dummy_server = DummyScheduleServer::new(8002).await;

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

    crate::assert_future_in!(
        announcer.poll(),
        now_i + Duration::from_secs(3),
        AnnouncerPollResult::ScheduleRefreshed(AnnouncerScheduleChanges::NoChanges)
    );

    dummy_server.stop().await;
}

#[tokio::test]
async fn t3_changes_to_the_schedule_are_noticed() {
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

#[tokio::test]
async fn t4_basic_event_notification() {
    let mut dummy_server = DummyScheduleServer::new(8004).await;

    let now = Utc::now();
    let mut events = vec![
        Event::dummy(0, (now + ChronoDuration::try_seconds(1).unwrap()).into()),
        Event::dummy(1, (now + ChronoDuration::try_seconds(2).unwrap()).into()),
        Event::dummy(2, (now + ChronoDuration::try_seconds(3).unwrap()).into()),
    ];

    dummy_server.set_events(events.clone());

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

#[tokio::test]
async fn t5_event_notification_with_multiple_identical_start_times() {
    let mut dummy_server = DummyScheduleServer::new(8005).await;

    let now = Utc::now();
    let mut events = vec![
        Event::dummy(0, (now + ChronoDuration::try_seconds(1).unwrap()).into()),
        Event::dummy(1, (now + ChronoDuration::try_seconds(2).unwrap()).into()),
        Event::dummy(2, (now + ChronoDuration::try_seconds(2).unwrap()).into()),
        Event::dummy(3, (now + ChronoDuration::try_seconds(3).unwrap()).into()),
    ];

    dummy_server.set_events(events.clone());

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
        now_i + Duration::from_secs(2),
        AnnouncerPollResult::Event(events[2].clone())
    );

    crate::assert_future_in!(
        announcer.poll(),
        now_i + Duration::from_secs(3),
        AnnouncerPollResult::Event(events[3].clone())
    );

    dummy_server.stop().await;
}

#[tokio::test]
async fn t6_basic_event_notification_unsorted() {
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

#[tokio::test]
async fn t7_event_notification_with_schedule_update() {
    let mut dummy_server = DummyScheduleServer::new(8007).await;

    let now = Utc::now();
    let mut events = vec![
        Event::dummy(0, (now + ChronoDuration::try_seconds(1).unwrap()).into()),
        Event::dummy(1, (now + ChronoDuration::try_seconds(3).unwrap()).into()),
        Event::dummy(2, (now + ChronoDuration::try_seconds(7).unwrap()).into()),
    ];

    dummy_server.set_events(vec![
        events[0].clone(),
        events[1].clone(),
        events[2].clone(),
    ]);

    fixup_events_for_test_comparison(&mut events);

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

    crate::assert_future_in!(
        announcer.poll(),
        now_i + Duration::from_secs(6),
        AnnouncerPollResult::ScheduleRefreshed(AnnouncerScheduleChanges::NoChanges)
    );

    crate::assert_future_in!(
        announcer.poll(),
        now_i + Duration::from_secs(7),
        AnnouncerPollResult::Event(events[2].clone())
    );

    crate::assert_future_in!(
        announcer.poll(),
        now_i + Duration::from_secs(8),
        AnnouncerPollResult::ScheduleRefreshed(AnnouncerScheduleChanges::NoChanges)
    );

    dummy_server.stop().await;
}
