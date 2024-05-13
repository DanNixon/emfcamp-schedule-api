use super::*;

#[test]
fn get_next_to_announce_initial_zero_offset() {
    let epoch = Utc::now();

    let events = vec![
        Event::dummy(
            0,
            (epoch + ChronoDuration::try_seconds(-10).unwrap()).into(),
        ),
        Event::dummy(1, (epoch + ChronoDuration::try_seconds(0).unwrap()).into()),
        Event::dummy(2, (epoch + ChronoDuration::try_seconds(10).unwrap()).into()),
        Event::dummy(3, (epoch + ChronoDuration::try_seconds(20).unwrap()).into()),
        Event::dummy(4, (epoch + ChronoDuration::try_seconds(30).unwrap()).into()),
    ];

    let offset = ChronoDuration::zero();

    let t = epoch + ChronoDuration::try_seconds(5).unwrap();
    let next = get_next_event_to_announce(&events, offset, &None, t.into());
    assert_eq!(next, Some(events[2].clone()));
}

#[test]
fn get_next_to_announce_initial_with_offset() {
    let epoch = Utc::now();

    let events = vec![
        Event::dummy(
            0,
            (epoch + ChronoDuration::try_seconds(-10).unwrap()).into(),
        ),
        Event::dummy(1, (epoch + ChronoDuration::try_seconds(0).unwrap()).into()),
        Event::dummy(2, (epoch + ChronoDuration::try_seconds(10).unwrap()).into()),
        Event::dummy(3, (epoch + ChronoDuration::try_seconds(20).unwrap()).into()),
        Event::dummy(4, (epoch + ChronoDuration::try_seconds(30).unwrap()).into()),
    ];

    let offset = ChronoDuration::try_seconds(-10).unwrap();

    let t = epoch + ChronoDuration::try_seconds(5).unwrap();
    let next = get_next_event_to_announce(&events, offset, &None, t.into());
    assert_eq!(next, Some(events[3].clone()));
}
