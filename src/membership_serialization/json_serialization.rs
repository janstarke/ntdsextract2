use serde::{ser::SerializeSeq, Deserialize};

use crate::{win32_types::Rdn, MembershipSet, SerializationType};

use super::Membership;
pub struct JsonSerialization;

impl SerializationType for JsonSerialization {
    fn serialize<S>(
        items: impl Iterator<Item = Option<String>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut ser = serializer.serialize_seq(None)?;
        for item in items {
            ser.serialize_element(&item)?;
        }
        ser.end()
    }

    fn deserialize<'de, D>(deserializer: D) -> Result<MembershipSet<Self>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = serde_json::Value::deserialize(deserializer)?;

        match v {
            serde_json::Value::Null => Ok(vec![].into_iter().into()),
            serde_json::Value::Bool(b) => Ok(vec![Membership::<Self>::from(
                Rdn::try_from(format!("{b}")).unwrap(),
            )]
            .into_iter()
            .into()),
            serde_json::Value::Number(n) => Ok(vec![Membership::<Self>::from(
                Rdn::try_from(format!("{n}")).unwrap(),
            )]
            .into_iter()
            .into()),
            serde_json::Value::String(s) => {
                Ok(vec![Membership::<Self>::from(Rdn::try_from(s).unwrap())]
                    .into_iter()
                    .into())
            }
            serde_json::Value::Array(a) => {
                let mut values = Vec::new();
                for v in a.into_iter() {
                    values.push(Membership::<Self>::from(
                        Rdn::try_from(v.to_string()).unwrap(),
                    ));
                }
                Ok(values.into_iter().into())
            }
            serde_json::Value::Object(_) => panic!("unexpected type: object"),
        }
    }
}
