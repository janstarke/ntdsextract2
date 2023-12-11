use serde::{ser::SerializeSeq, Deserialize};

use crate::{SerializationType, StringSet};
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
            serde_json::Value::Null => Ok(Vec::<String>::new().into()),
            serde_json::Value::Bool(b) => Ok(vec![format!("{b}")].into()),
            serde_json::Value::Number(n) => Ok(vec![format!("{n}")].into()),
            serde_json::Value::String(s) => Ok(vec![s].into()),
            serde_json::Value::Array(a) => {
                let v: Vec<_> = a.into_iter().map(|s| s.to_string()).collect();
                Ok(v.into())
            }
            serde_json::Value::Object(_) => panic!("unexpected type: object"),
        }
    }
}
