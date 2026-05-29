mod snapshot;

pub use snapshot::TimeSnapshot;

#[must_use]
pub fn now_utc() -> TimeSnapshot {
    TimeSnapshot::now_utc()
}

#[must_use]
pub fn now_local() -> TimeSnapshot {
    TimeSnapshot::now_local()
}

#[must_use]
pub fn unix_timestamp() -> i64 {
    TimeSnapshot::now_utc().unix_seconds()
}

#[must_use]
pub fn unix_timestamp_millis() -> i64 {
    TimeSnapshot::now_utc().unix_millis()
}