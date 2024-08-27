use chrono::{DateTime, Duration, Utc};

use crate::impl_timestamp;

use super::BASE_DATETIME;

#[derive(Eq, PartialEq)]
pub struct TruncatedWindowsFileTime(DateTime<Utc>);

impl_timestamp!(TruncatedWindowsFileTime);

impl From<i64> for TruncatedWindowsFileTime {
    fn from(value: i64) -> Self {
        Self(*BASE_DATETIME + Duration::seconds(value))
    }
}
impl From<DateTime<Utc>> for TruncatedWindowsFileTime {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}