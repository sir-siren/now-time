# Changelog

## 1.0.0 -- initial release

- `TimeSnapshot::now_utc()` -- current UTC time
- `TimeSnapshot::now_local()` -- current local time
- `TimeSnapshot::unix_seconds()` -- Unix timestamp in seconds
- `TimeSnapshot::unix_millis()` -- Unix timestamp in milliseconds
- `TimeSnapshot::to_rfc3339()` -- ISO 8601 UTC string
- `TimeSnapshot::to_local_rfc3339()` -- ISO 8601 with local offset
- `TimeSnapshot::to_rfc2822()` -- RFC 2822 string
- `TimeSnapshot::format(pattern)` -- custom strftime-style format
- `now_utc()`, `now_local()`, `unix_timestamp()`, `unix_timestamp_millis()` free functions
