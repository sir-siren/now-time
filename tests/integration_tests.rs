// tests/ runs as a separate compilation unit -- uses the public API only.
// this is exactly how a downstream crate would use now-time.

use now_time::{TimeSnapshot, now_local, now_utc, unix_timestamp, unix_timestamp_millis};

#[test]
fn now_utc_convenience_fn_returns_same_second_as_snapshot() {
    let via_fn = now_utc().unix_seconds();
    let via_struct = TimeSnapshot::now_utc().unix_seconds();
    assert!((via_fn - via_struct).abs() <= 1);
}

#[test]
fn unix_timestamp_matches_now_utc_unix_seconds() {
    let from_fn = unix_timestamp();
    let from_snapshot = TimeSnapshot::now_utc().unix_seconds();
    assert!((from_fn - from_snapshot).abs() <= 1);
}

#[test]
fn unix_timestamp_millis_is_greater_than_unix_timestamp_times_1000() {
    // millis must be at least seconds * 1000 (the sub-second part is non-negative)
    let ms = unix_timestamp_millis();
    let s = unix_timestamp();
    assert!(ms >= s * 1_000);
    assert!(ms < (s + 1) * 1_000);
}

#[test]
fn now_local_and_now_utc_are_within_one_second() {
    let utc = now_utc().unix_seconds();
    let local = now_local().unix_seconds();
    assert!((utc - local).abs() <= 1);
}

#[test]
fn rfc3339_string_is_parseable_as_utc() {
    let s = now_utc().to_rfc3339();
    // a well-formed rfc3339 string has a 'T' separator and a '+' or 'Z' offset
    assert!(s.contains('T'), "rfc3339 must contain T separator: {s}");
    assert!(s.ends_with("+00:00"), "utc rfc3339 must end with +00:00: {s}");
}

#[test]
fn rfc2822_string_ends_with_utc_offset() {
    let s = now_utc().to_rfc2822();
    assert!(s.ends_with("+0000"), "utc rfc2822 must end with +0000: {s}");
}

#[test]
fn display_impl_is_same_as_to_rfc3339() {
    let t = now_utc();
    assert_eq!(format!("{t}"), t.to_rfc3339());
}

#[test]
fn format_year_is_four_digits() {
    let year = now_utc().format("%Y").expect("year format must not fail");
    assert_eq!(year.len(), 4);
    assert!(year.chars().all(|c| c.is_ascii_digit()));
}

#[test]
fn format_ymd_is_ten_chars() {
    let date = now_utc().format("%Y-%m-%d").expect("ymd format must not fail");
    assert_eq!(date.len(), 10);
}

#[test]
fn snapshots_taken_in_order_compare_correctly() {
    let first = now_utc();
    // tiny spin -- enough for the clock to tick at least one nanosecond
    std::thread::sleep(std::time::Duration::from_millis(5));
    let second = now_utc();
    assert!(second >= first, "later snapshot must not be less than earlier one");
}