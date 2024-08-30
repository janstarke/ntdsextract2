use serde::Deserialize;

use crate::{win32_types::Rdn, MembershipSet, SerializationType};

use super::Membership;

pub struct CsvSerialization;

impl SerializationType for CsvSerialization {
    fn serialize<S>(
        items: impl Iterator<Item = Option<String>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let v = Vec::from_iter(items.map(|i| match i {
            Some(i) => i,
            None => "".to_owned(),
        }))
        .join(",");
        serializer.serialize_str(&v)
    }

    fn deserialize<'de, D>(deserializer: D) -> Result<MembershipSet<Self>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut parts = Vec::new();
        for s in s.split(',') {
            parts.push(Membership::<Self>::from(Rdn::try_from(s).unwrap()))
        }
        Ok(MembershipSet::<Self>::from(parts.into_iter()))
    }
}
