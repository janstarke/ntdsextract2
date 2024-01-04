use serde::Deserialize;

use crate::{SerializationType, RdnSet, win32_types::Rdn};

pub struct CsvSerialization;

impl SerializationType for CsvSerialization {
    fn serialize<S>(
        items: impl Iterator<Item = String>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let v = Vec::from_iter(items).join(",");
        serializer.serialize_str(&v)
    }

    fn deserialize<'de, D>(deserializer: D) -> Result<RdnSet<Self>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut parts = Vec::new();
        for s in s.split(',') {
            parts.push(Rdn::try_from(s).unwrap())
        }
        Ok(parts.into())
    }
}
