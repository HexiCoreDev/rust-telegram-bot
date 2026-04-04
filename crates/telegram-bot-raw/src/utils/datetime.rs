use chrono::{DateTime, Utc};

/// Converts a Unix timestamp (seconds) to a UTC `DateTime`.
pub fn from_timestamp(unixtime: i64) -> Option<DateTime<Utc>> {
    DateTime::from_timestamp(unixtime, 0)
}

/// Converts a UTC `DateTime` to a Unix timestamp (seconds).
pub fn to_timestamp(dt: &DateTime<Utc>) -> i64 {
    dt.timestamp()
}

/// The Unix epoch (1970-01-01 00:00:00 UTC), equivalent to `ZERO_DATE` in Python.
pub fn zero_date() -> DateTime<Utc> {
    DateTime::from_timestamp(0, 0).expect("epoch is always valid")
}
