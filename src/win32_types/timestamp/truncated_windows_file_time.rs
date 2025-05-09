use chrono::{DateTime, Duration, Utc};
use lazy_static::lazy_static;
use libesedb::systemtime_from_filetime;

use crate::impl_timestamp;

#[derive(Eq, PartialEq)]
pub struct TruncatedWindowsFileTime(DateTime<Utc>);

impl_timestamp!(TruncatedWindowsFileTime);

lazy_static!{
    static ref BASE_TIME: DateTime<Utc> = systemtime_from_filetime(0).into();
}

impl From<u64> for TruncatedWindowsFileTime {
    fn from(value: u64) -> Self {
        Self(*BASE_TIME + Duration::seconds(value.try_into().unwrap()))
    }
}
impl From<DateTime<Utc>> for TruncatedWindowsFileTime {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}
impl From<TruncatedWindowsFileTime> for DateTime<Utc> {
    fn from(value: TruncatedWindowsFileTime) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use crate::win32_types::TruncatedWindowsFileTime;

    #[test]
    fn convert_from_ntds() {
        let ds_record_time = 13390472401u64;
        let ts = TruncatedWindowsFileTime::from(ds_record_time);
        assert_eq!(ts.0.to_rfc3339(), "2025-04-30T07:40:01+00:00");
    }
}