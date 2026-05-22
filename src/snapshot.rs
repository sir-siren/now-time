use chrono::{DateTime, Local, Utc};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeSnapshot {
    inner: DateTime<Utc>,
}

impl TimeSnapshot {
    pub fn now_utc() -> Self {
        Self { inner: Utc::now() }
    }

    /// Captures the current time. Internally stored as UTC; local offset is
    /// applied at output time via `to_local_rfc3339()`.
    pub fn now_local() -> Self {
        Self { inner: Utc::now() }
    }

    pub fn unix_seconds(&self) -> i64 {
        self.inner.timestamp()
    }

    pub fn unix_millis(&self) -> i64 {
        self.inner.timestamp_millis()
    }

    pub fn to_rfc3339(&self) -> String {
        self.inner.to_rfc3339()
    }

    pub fn to_local_rfc3339(&self) -> String {
        let local: DateTime<Local> = self.inner.into();
        local.to_rfc3339()
    }

    pub fn to_rfc2822(&self) -> String {
        self.inner.to_rfc2822()
    }

    pub fn format(&self, pattern: &str) -> String {
        self.inner.format(pattern).to_string()
    }

    pub fn as_datetime(&self) -> &DateTime<Utc> {
        &self.inner
    }
}

impl std::fmt::Display for TimeSnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_rfc3339())
    }
}

#[cfg(test)]
mod tests {
    use super::TimeSnapshot;

    #[test]
    fn now_utc_unix_seconds_is_after_year_2023() {
        assert!(TimeSnapshot::now_utc().unix_seconds() > 1_700_000_000);
    }

    #[test]
    fn unix_millis_is_one_thousand_times_seconds() {
        let t = TimeSnapshot::now_utc();
        let diff = t.unix_millis() - t.unix_seconds() * 1_000;
        assert!((0..1_000).contains(&diff));
    }

    #[test]
    fn to_rfc3339_ends_with_utc_offset() {
        let s = TimeSnapshot::now_utc().to_rfc3339();
        assert!(s.ends_with("+00:00"), "rfc3339 utc must end with +00:00, got: {s}");
    }

    #[test]
    fn to_rfc2822_ends_with_utc_offset() {
        let s = TimeSnapshot::now_utc().to_rfc2822();
        assert!(s.ends_with("+0000"), "rfc2822 utc must end with +0000, got: {s}");
    }

    #[test]
    fn format_year_returns_four_digit_string() {
        let year = TimeSnapshot::now_utc().format("%Y");
        assert_eq!(year.len(), 4, "year must be 4 digits, got: {year}");
        assert!(year.parse::<u32>().is_ok(), "year must be numeric, got: {year}");
    }

    #[test]
    fn format_date_returns_ten_char_string() {
        let date = TimeSnapshot::now_utc().format("%Y-%m-%d");
        assert_eq!(date.len(), 10, "YYYY-MM-DD must be 10 chars, got: {date}");
    }

    #[test]
    fn display_impl_matches_rfc3339() {
        let t = TimeSnapshot::now_utc();
        assert_eq!(format!("{t}"), t.to_rfc3339());
    }

    #[test]
    fn now_utc_and_now_local_are_within_one_second_of_each_other() {
        let utc = TimeSnapshot::now_utc().unix_seconds();
        let local = TimeSnapshot::now_local().unix_seconds();
        assert!((utc - local).abs() <= 1);
    }
}