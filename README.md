# now-time

**Get the current UTC or local time with zero boilerplate.**

[![Rust](https://img.shields.io/badge/Rust-1.85+-DEA584?style=flat&logo=rust&logoColor=FF9170)](https://www.rust-lang.org)
[![crates.io](https://img.shields.io/crates/v/now-time?style=flat)](https://crates.io/crates/now-time)
[![docs.rs](https://img.shields.io/docsrs/now-time?style=flat)](https://docs.rs/now-time)
[![License](https://img.shields.io/badge/license-MIT-AEC6CF?style=flat)](./LICENSE)

## Overview

`now-time` gives you the current time in one line. No setup, no config, no boilerplate.

Two dependencies: `chrono` for the time primitives, `thiserror` for errors. Everything else is stdlib.

## Install

```toml
[dependencies]
now-time = "1.2.0"
```

## Usage

### Quick functions

```rust
use now_time::{now_utc, now_local, unix_timestamp, unix_timestamp_millis};

println!("{}", now_utc());                // 2024-11-15T10:30:00.000000000+00:00
println!("{}", now_local());              // 2024-11-15T18:30:00.000000000+08:00
println!("{}", unix_timestamp());         // 1731664200
println!("{}", unix_timestamp_millis());  // 1731664200123
```

### Full API

```rust
use now_time::TimeSnapshot;

let t = TimeSnapshot::now_utc();

println!("{}", t.to_rfc3339());                       // ISO 8601 UTC
println!("{}", t.to_local_rfc3339());                 // ISO 8601 with local offset
println!("{}", t.to_rfc2822());                       // email / HTTP header style
println!("{}", t.unix_seconds());                     // 1731664200
println!("{}", t.unix_millis());                      // 1731664200123
println!("{}", t.format("%Y-%m-%d %H:%M:%S"));        // 2024-11-15 10:30:00
```

### Custom format

Uses chrono's strftime specifiers:

| Spec | Meaning        | Example |
| ---- | -------------- | ------- |
| `%Y` | 4-digit year   | `2024`  |
| `%m` | month (01-12)  | `11`    |
| `%d` | day (01-31)    | `15`    |
| `%H` | hour (00-23)   | `10`    |
| `%M` | minute (00-59) | `30`    |
| `%S` | second (00-59) | `00`    |

Full list: https://docs.rs/chrono/latest/chrono/format/strftime

## API Reference

| Function / Method                  | Returns          | Description                    |
| ---------------------------------- | ---------------- | ------------------------------ |
| `now_utc()`                        | `TimeSnapshot`   | Current time in UTC            |
| `now_local()`                      | `TimeSnapshot`   | Current time in local timezone |
| `unix_timestamp()`                 | `i64`            | Unix timestamp in seconds      |
| `unix_timestamp_millis()`          | `i64`            | Unix timestamp in milliseconds |
| `TimeSnapshot::to_rfc3339()`       | `String`         | ISO 8601 UTC string            |
| `TimeSnapshot::to_local_rfc3339()` | `String`         | ISO 8601 with local offset     |
| `TimeSnapshot::to_rfc2822()`       | `String`         | RFC 2822 string                |
| `TimeSnapshot::unix_seconds()`     | `i64`            | Seconds since Unix epoch       |
| `TimeSnapshot::unix_millis()`      | `i64`            | Milliseconds since Unix epoch  |
| `TimeSnapshot::format(pattern)`    | `String`         | Custom strftime format         |
| `TimeSnapshot::as_datetime()`      | `&DateTime<Utc>` | Raw chrono DateTime            |

## License

MIT. See [LICENSE](./LICENSE).