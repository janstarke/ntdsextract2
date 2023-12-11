use serde::Deserialize;

use crate::{SerializationType, StringSet};

pub struct CsvSerialization;

impl SerializationType for CsvSerialization {
    fn serialize<'a, S>(
        items: impl Iterator<Item = &'a str>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let v = Vec::from_iter(items).join(",");
        serializer.serialize_str(&v)
    }

    fn deserialize<'de, D>(deserializer: D) -> Result<StringSet<Self>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let parts: Vec<_> = s.split(',').collect();
        Ok(parts.into())
    }
}
