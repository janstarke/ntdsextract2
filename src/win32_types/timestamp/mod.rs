use chrono::{DateTime, NaiveDate, Utc};
use lazy_static::lazy_static;

mod database_time;
mod windows_file_time;
mod truncated_windows_file_time;
mod utc_visitor;

use utc_visitor::*;
pub use database_time::DatabaseTime;
pub use windows_file_time::WindowsFileTime;
pub use truncated_windows_file_time::TruncatedWindowsFileTime;

lazy_static! {
    static ref BASE_DATETIME: DateTime<Utc> = DateTime::<Utc>::from_utc(
        NaiveDate::from_ymd_opt(1601, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
        Utc
    );
}

pub trait ToRfc3339 {
    fn to_rfc3339(&self) -> String;
}

#[macro_export]
macro_rules! impl_timestamp {
    ($type: ident) => {
        impl $crate::esedb_utils::FromValue for $type {
            fn from_value_opt(
                value: &libesedb::Value,
                attribute_name: &str,
            ) -> anyhow::Result<Option<Self>> {
                match value {
                    libesedb::Value::Currency(val) => Ok(Some($type::from(*val))),
                    libesedb::Value::Null(()) => Ok(None),
                    _ => Err(anyhow::anyhow!(
                        "invalid value detected: {:?} in field {}",
                        value,
                        attribute_name
                    )),
                }
            }
        }

        impl $type {
            #[allow(dead_code)]
            pub fn timestamp(&self) -> i64 {
                self.0.timestamp()
            }

            pub fn from_ts<'de, D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                Ok(Self::from($crate::serialization::from_ts(deserializer)?))
            }
        }

        impl $crate::win32_types::ToRfc3339 for $type {
            fn to_rfc3339(&self) -> String {
                self.0.to_rfc3339()
            }
        }

        impl<'de> serde::Deserialize<'de> for $type {
            fn deserialize<D>(deserializer: D) -> std::prelude::v1::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let me = deserializer.deserialize_str($crate::win32_types::timestamp::UtcVisitor::default())?;
                Ok(Self::from(me))
            }
        }
    };
}
