use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeDelta};
use serde::{Deserialize, Deserializer};

pub(super) fn deserialize<'de, D>(
    deserializer: D,
) -> std::result::Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    parse_timestamp_string(&s).map_err(serde::de::Error::custom)
}

fn parse_timestamp_string(s: &str) -> Result<DateTime<FixedOffset>, &str> {
    match DateTime::parse_from_rfc3339(s) {
        Ok(t) => Ok(t),
        Err(_) => {
            match NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
                Ok(t) => {
                    // Fix broken by design timestamping on the emfcamp schedule API.
                    //
                    // It's almost as if someone needs to come up with a standard way of representing
                    // time on the internet to avoid issues like this. /s
                    // But really, every programming language includes a library capable of handling RFC 3339 and RFC
                    // 2822, what possible reason is there for formatting timestamps in anything different.
                    // At least chrono makes correcting this fuckup easy.
                    Ok(DateTime::from_naive_utc_and_offset(
                        t - TimeDelta::try_hours(1).unwrap(),
                        FixedOffset::east_opt(60 * 60).unwrap(),
                    ))
                }
                Err(_) => Err("Failed to parse any timestamp format"),
            }
        }
    }
}
