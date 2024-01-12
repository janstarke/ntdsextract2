use chrono::{DateTime, Utc, NaiveDate, NaiveTime, NaiveDateTime};

use crate::impl_timestamp;

#[derive(Eq, PartialEq)]
pub struct DatabaseTime(DateTime<Utc>);
impl_timestamp!(DatabaseTime);

impl From<i64> for DatabaseTime {
    fn from(value: i64) -> Self {
        let bytes = value.to_le_bytes();
        let date = match NaiveDate::from_ymd_opt(
            std::convert::Into::<i32>::into(bytes[5]) + 1900,
            bytes[4].into(),
            bytes[3].into(),
        ) {
            Some(val) => val,
            None => panic!("unable to convert '{bytes:?}' into a database date"),
        };

        let time = match NaiveTime::from_hms_opt(bytes[2].into(), bytes[1].into(), bytes[0].into())
        {
            Some(val) => val,
            None => panic!("unable to convert '{bytes:?}' into a database time"),
        };
        let ts = DateTime::<Utc>::from_utc(NaiveDateTime::new(date, time), Utc);
        Self(ts)
    }
}

impl From<DateTime<Utc>> for DatabaseTime {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}