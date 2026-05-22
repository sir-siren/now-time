pub mod errors;
mod snapshot;

pub use errors::TimeError;
pub use snapshot::TimeSnapshot;

pub fn now_utc() -> TimeSnapshot {
    TimeSnapshot::now_utc()
}

pub fn now_local() -> TimeSnapshot {
    TimeSnapshot::now_local()
}

pub fn unix_timestamp() -> i64 {
    TimeSnapshot::now_utc().unix_seconds()
}

pub fn unix_timestamp_millis() -> i64 {
    TimeSnapshot::now_utc().unix_millis()
}