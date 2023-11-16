use chrono::{DateTime, Utc};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serializer};

use crate::do_flat_serialization;
use crate::win32_types::ToRfc3339;

pub(crate) fn to_ts<T, S>(ts: &Option<T>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: ToRfc3339,
{
    match ts {
        Some(ts) => s.serialize_str(&ts.to_rfc3339()),
        None => s.serialize_str(""),
    }
}

pub fn from_ts<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de;
    let buf = String::deserialize(deserializer)?;
    match DateTime::parse_from_rfc3339(&buf[..]) {
        Ok(dt) => Ok(dt.with_timezone(&Utc)),
        Err(why) => Err(de::Error::custom(format!(
            "unable to parse timestamp '{buf}': {why}"
        ))),
    }
}

pub(crate) fn serialize_object_list<S>(ol: &[String], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if do_flat_serialization() {
        s.serialize_str(&ol.join(","))
    } else {
        let mut seq = s.serialize_seq(None)?;
        for o in ol.iter() {
            seq.serialize_element(o)?;
        }
        seq.end()
    }
}
