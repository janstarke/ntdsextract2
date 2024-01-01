use serde::{ser::SerializeSeq, Deserialize};

use crate::{win32_types::NameWithGuid, SerializationType, StringSet};
pub struct JsonSerialization;

impl SerializationType for JsonSerialization {
    fn serialize<'a, S>(
        items: impl Iterator<Item = &'a str>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut ser = serializer.serialize_seq(None)?;
        for item in items {
            ser.serialize_element(item)?;
        }
        ser.end()
    }

    fn deserialize<'de, D>(deserializer: D) -> Result<StringSet<Self>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = serde_json::Value::deserialize(deserializer)?;

        match v {
            serde_json::Value::Null => Ok(Vec::<NameWithGuid>::new().into()),
            serde_json::Value::Bool(b) => Ok(vec![NameWithGuid::try_from(format!("{b}")).unwrap()].into()),
            serde_json::Value::Number(n) => Ok(vec![NameWithGuid::try_from(format!("{n}")).unwrap()].into()),
            serde_json::Value::String(s) => Ok(vec![NameWithGuid::try_from(s).unwrap()].into()),
            serde_json::Value::Array(a) => {
                let mut values = Vec::new();
                for v in a.into_iter() {
                    values.push(NameWithGuid::try_from(v.to_string()).unwrap());
                }
                Ok(values.into())
            }
            serde_json::Value::Object(_) => panic!("unexpected type: object"),
        }
    }
}
