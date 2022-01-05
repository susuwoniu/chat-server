use chrono::{offset::FixedOffset, DateTime, NaiveDateTime, Utc};

#[allow(dead_code)]
pub fn naive_to_utc(naive: NaiveDateTime) -> DateTime<Utc> {
    DateTime::from_utc(naive, Utc)
}

pub fn naive_to_beijing(naive: NaiveDateTime) -> DateTime<FixedOffset> {
    DateTime::from_utc(naive, FixedOffset::east(8 * 3600))
}
