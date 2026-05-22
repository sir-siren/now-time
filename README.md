# now-time

Get the current UTC or local time with zero boilerplate.

## Install

```toml
[dependencies]
now-time = "1.0.0"
```

## Usage

```rust
use now_time::{now_utc, now_local, unix_timestamp};

// one-liners
println!("{}", now_utc());         // 2024-11-15T10:30:00.000000000+00:00
println!("{}", now_local());       // 2024-11-15T18:30:00.000000000+08:00
println!("{}", unix_timestamp());  // 1731664200

// full API
use now_time::TimeSnapshot;

let t = TimeSnapshot::now_utc();
println!("{}", t.to_rfc3339());               // ISO 8601 UTC
println!("{}", t.to_local_rfc3339());         // ISO 8601 with local offset
println!("{}", t.to_rfc2822());               // email / HTTP header style
println!("{}", t.unix_seconds());             // 1731664200
println!("{}", t.unix_millis());              // 1731664200123
println!("{}", t.format("%Y-%m-%d").unwrap()); // 2024-11-15
```

## License

MIT OR Apache-2.0
