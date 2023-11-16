use chrono::{DateTime, Utc, Duration};

use crate::impl_timestamp;

use super::BASE_DATETIME;


#[derive(Eq, PartialEq)]
pub struct WindowsFileTime(DateTime<Utc>);

impl_timestamp!(WindowsFileTime);

impl From<i64> for WindowsFileTime {
    fn from(value: i64) -> Self {
        Self(*BASE_DATETIME + Duration::microseconds(value / 10))
    }
}
impl From<DateTime<Utc>> for WindowsFileTime {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}