use chrono::{DateTime, Utc};
use libesedb::systemtime_from_filetime;

use crate::impl_timestamp;

#[derive(Eq, PartialEq)]
pub struct WindowsFileTime(DateTime<Utc>);

impl_timestamp!(WindowsFileTime);

impl From<u64> for WindowsFileTime {
    fn from(value: u64) -> Self {
        Self(systemtime_from_filetime(value).into())
    }
}
impl From<DateTime<Utc>> for WindowsFileTime {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}
