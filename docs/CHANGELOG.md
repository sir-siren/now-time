# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.3] - 2026-05-22

### Changed
- Bump minimum Rust version badge in README to 1.95+
- Remove dead `TimeError` / `errors` module from public API

### Fixed
- Version mismatch between `Cargo.toml`, `README.md` badge, and install snippet
- `now_local()` missing doc comment clarifying internal UTC storage

## [1.0.0] - 2025-01-01

### Added
- `TimeSnapshot::now_utc()` — current UTC time
- `TimeSnapshot::now_local()` — current local time
- `TimeSnapshot::unix_seconds()` — Unix timestamp in seconds
- `TimeSnapshot::unix_millis()` — Unix timestamp in milliseconds
- `TimeSnapshot::to_rfc3339()` — ISO 8601 UTC string
- `TimeSnapshot::to_local_rfc3339()` — ISO 8601 with local offset
- `TimeSnapshot::to_rfc2822()` — RFC 2822 string
- `TimeSnapshot::format(pattern)` — custom strftime-style format
- `TimeSnapshot::as_datetime()` — raw `&DateTime<Utc>` access
- Free functions: `now_utc()`, `now_local()`, `unix_timestamp()`, `unix_timestamp_millis()`