use anyhow::anyhow;
use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use libesedb::Value;

use crate::esedb_utils::FromValue;

pub trait ToRfc3339 {
    fn to_rfc3339(&self) -> String;
}

#[derive(Eq, PartialEq)]
pub struct DatabaseTime(DateTime<Utc>);

#[derive(Eq, PartialEq)]
pub struct WindowsFileTime(DateTime<Utc>);

#[derive(Eq, PartialEq)]
pub struct TruncatedWindowsFileTime(DateTime<Utc>);

impl From<i64> for DatabaseTime {
    fn from(value: i64) -> Self {
        let bytes = value.to_le_bytes();
        let date = NaiveDate::from_ymd_opt(
            std::convert::Into::<i32>::into(bytes[5]) + 1900,
            bytes[4].into(),
            bytes[3].into(),
        )
        .unwrap();
        let time = match NaiveTime::from_hms_opt(bytes[2].into(), bytes[1].into(), bytes[0].into())
        {
            Some(val) => val,
            None => panic!("unable to convert '{value}' into a database timestamp"),
        };
        let ts = DateTime::<Utc>::from_utc(NaiveDateTime::new(date, time), Utc);
        Self(ts)
    }
}

impl From<i64> for WindowsFileTime {
    fn from(value: i64) -> Self {
        let dt_base =
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(1601, 1, 1).and_hms(0, 0, 0), Utc);
        let duration = Duration::microseconds(value / 10);
        Self(dt_base + duration)
    }
}

impl From<i64> for TruncatedWindowsFileTime {
    fn from(value: i64) -> Self {
        let dt_base =
            DateTime::<Utc>::from_utc(NaiveDate::from_ymd(1601, 1, 1).and_hms(0, 0, 0), Utc);
        let duration = Duration::seconds(value);
        Self(dt_base + duration)
    }
}

impl From<DateTime<Utc>> for DatabaseTime {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<DateTime<Utc>> for WindowsFileTime {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<DateTime<Utc>> for TruncatedWindowsFileTime {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

macro_rules! impl_timestamp {
    ($type: ident) => {
        impl FromValue for $type {
            fn from_value_opt(
                value: libesedb::Value,
                attribute_name: &str,
            ) -> anyhow::Result<Option<Self>> {
                match value {
                    Value::Currency(val) => Ok(Some($type::from(val))),
                    Value::Null => Ok(None),
                    _ => Err(anyhow!(
                        "invalid value detected: {:?} in field {}",
                        value,
                        attribute_name
                    )),
                }
            }
        }

        impl $type {
            pub fn timestamp(&self) -> i64 {
                self.0.timestamp()
            }
        }

        impl ToRfc3339 for $type {
            fn to_rfc3339(&self) -> String {
                self.0.to_rfc3339()
            }
        }
    };
}
impl_timestamp!(DatabaseTime);
impl_timestamp!(WindowsFileTime);
impl_timestamp!(TruncatedWindowsFileTime);
