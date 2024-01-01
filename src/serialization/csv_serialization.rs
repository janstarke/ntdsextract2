use serde::Deserialize;

use crate::{SerializationType, StringSet, win32_types::NameWithGuid};

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
        let mut parts = Vec::new();
        for s in s.split(',') {
            parts.push(NameWithGuid::try_from(s).unwrap())
        }
        Ok(parts.into())
    }
}
