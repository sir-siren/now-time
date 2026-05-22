use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum TimeError {
    #[error("system clock is behind the Unix epoch -- check your system clock")]
    ClockBeforeEpoch,

    #[error("invalid format pattern '{pattern}'")]
    InvalidFormatPattern {
        pattern: String,
    },
}