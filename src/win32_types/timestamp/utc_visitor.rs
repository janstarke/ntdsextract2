use chrono::{DateTime, Utc};
use serde::de::Visitor;

#[derive(Default)]
pub struct UtcVisitor ();

impl<'de> Visitor<'de> for UtcVisitor {
    type Value = DateTime<Utc>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "an RFC3339 compliant timestamp")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match DateTime::parse_from_rfc3339(v) {
            Ok(dt) => Ok(dt.with_timezone(&Utc)),
            Err(why) => Err(E::custom(format!("timestamp '{v}' is invalid: {why}"))),
        }
    }
}
