use chrono::{format::StrftimeItems, DateTime, FixedOffset, NaiveDate, Utc};
use lazy_static::lazy_static;

mod database_time;
mod timeline_entry;
mod truncated_windows_file_time;
mod unix_timestamp;
mod windows_file_time;

pub use timeline_entry::*;
pub use truncated_windows_file_time::TruncatedWindowsFileTime;
pub use unix_timestamp::*;
pub use windows_file_time::WindowsFileTime;

lazy_static! {
    static ref BASE_DATETIME: DateTime<Utc> = DateTime::<Utc>::from_naive_utc_and_offset(
        NaiveDate::from_ymd_opt(1601, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
        Utc
    );
}

lazy_static! {
    pub static ref TIMESTAMP_FORMAT: String = {
        if let Ok(format) = std::env::var("DFIR_DATE") {
            if StrftimeItems::new(&format).any(|i| i == chrono::format::Item::Error) {
                eprintln!();
                eprintln!("ERROR: invalid date format: '{format}' stored in environment variable $DFIR_DATE!");
                eprintln!();
                eprintln!("Please take a look at");
                eprintln!();
                eprintln!(
                    "        <https://docs.rs/chrono/latest/chrono/format/strftime/index.html>"
                );
                eprintln!();
                eprintln!("to see which format strings are accepted.");
                eprintln!();
                std::process::exit(-1);
            } else {
                format
            }
        } else {
            // use this format, because to_rfc3339 creates values like '+30828-09-14T02:48:05.477580+00:00',
            // which cannot be parsed with parse_from_rfc3339()
            "%Y-%m-%dT%H:%M:%S%z".to_string()
        }
    };
    pub static ref ZERO: DateTime<FixedOffset> =
        DateTime::<FixedOffset>::parse_from_rfc3339("0000-00-00T00:00:00+00:00")
            .expect("unable to parse literal timestamp");
}

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
                    _ => Err($crate::ntds::Error::InvalidValueDetected(
                        value.to_string(),
                        stringify!($type),
                    )),
                }
            }
        }

        impl serde::Serialize for $type {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(
                    &self
                        .0
                        .format(&$crate::win32_types::timestamp::TIMESTAMP_FORMAT)
                        .to_string(),
                )
            }
        }

        impl<'de> serde::Deserialize<'de> for $type {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                use serde::de;
                let buf = String::deserialize(deserializer)?;
                match DateTime::parse_from_str(
                    &buf[..],
                    &$crate::win32_types::timestamp::TIMESTAMP_FORMAT,
                ) {
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
