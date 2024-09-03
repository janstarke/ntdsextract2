use chrono::{DateTime, Utc};
use libesedb::systemtime_from_filetime;

use crate::impl_timestamp;

#[derive(Eq, PartialEq)]
pub struct TruncatedWindowsFileTime(DateTime<Utc>);

impl_timestamp!(TruncatedWindowsFileTime);

impl From<u64> for TruncatedWindowsFileTime {
    fn from(value: u64) -> Self {
        Self(systemtime_from_filetime(value*10_000).into())
    }
}
impl From<DateTime<Utc>> for TruncatedWindowsFileTime {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}