use chrono::{DateTime, NaiveDate, Utc};
use lazy_static::lazy_static;

mod database_time;
mod truncated_windows_file_time;
mod windows_file_time;
mod unix_timestamp;
mod timeline_entry;

pub use truncated_windows_file_time::TruncatedWindowsFileTime;
pub use windows_file_time::WindowsFileTime;
pub use unix_timestamp::*;
pub use timeline_entry::*;

lazy_static! {
    static ref BASE_DATETIME: DateTime<Utc> = DateTime::<Utc>::from_utc(
        NaiveDate::from_ymd_opt(1601, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
        Utc
    );
}

static DATE_FORMAT: &str = "%d-%m-%YT%H:%M:%S%z";

#[macro_export]
macro_rules! impl_timestamp {
    ($type: ident) => {
        impl $crate::value::FromValue for $type {
            fn from_value_opt(
                value: &$crate::cache::Value,
            ) -> Result<Option<Self>, $crate::ntds::Error> {
                match value {
                    $crate::cache::Value::Currency(val) => Ok(Some($type::from(*val))),
                    $crate::cache::Value::Null(()) => Ok(None),
                    _ => Err($crate::ntds::Error::InvalidValueDetected(value.to_string(), stringify!($type))),
                }
            }
        }

        impl serde::Serialize for $type {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.0.format($crate::win32_types::timestamp::DATE_FORMAT).to_string())
            }
        }

        impl<'de> serde::Deserialize<'de> for $type {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de> {
                
            use serde::de;
            let buf = String::deserialize(deserializer)?;
            match DateTime::parse_from_str(&buf[..], $crate::win32_types::timestamp::DATE_FORMAT) {
                Ok(dt) => Ok(Self(dt.with_timezone(&Utc))),
                Err(why) => Err(de::Error::custom(format!(
                    "unable to parse timestamp '{buf}': {why}"
                ))),
            }
            }
        }

        impl $crate::win32_types::UnixTimestamp for $type {
            #[allow(dead_code)]
            fn timestamp(&self) -> i64 {
                self.0.timestamp()
            }
        }
        impl $crate::win32_types::TimelineEntry for $type {}
    };
}
