use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::{win32_types::Rdn, SerializationType};

pub struct RdnSet<T: SerializationType>(Vec<Rdn>, PhantomData<T>);

impl<'a, T> From<Vec<&'a Rdn>> for RdnSet<T>
where
    T: SerializationType,
{
    fn from(value: Vec<&'a Rdn>) -> Self {
        Self(value.into_iter().cloned().collect(), PhantomData)
    }
}

impl<T> From<Vec<Rdn>> for RdnSet<T>
where
    T: SerializationType,
{
    fn from(value: Vec<Rdn>) -> Self {
        Self(value, PhantomData)
    }
}

impl<T> Serialize for RdnSet<T>
where
    T: SerializationType,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        T::serialize(self.0.iter().map(|rdn| rdn.to_string()), serializer)
    }
}

impl<'de, T> Deserialize<'de> for RdnSet<T>
where
    T: SerializationType,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        T::deserialize(deserializer)
    }
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use crate::{win32_types::Rdn, CsvSerialization, JsonSerialization};

    use super::{RdnSet, SerializationType};

    #[derive(Serialize)]
    #[serde(bound = "T: SerializationType")]
    struct SampleRecord<T: SerializationType> {
        data: RdnSet<T>,
    }

    fn test_data<T>() -> SampleRecord<T>
    where
        T: SerializationType,
    {
        SampleRecord {
            data: RdnSet::<T>::from(vec![
                Rdn::try_from("a").unwrap(),
                Rdn::try_from("b").unwrap(),
                Rdn::try_from("c").unwrap(),
            ]),
        }
    }

    #[test]
    fn test_serialize_csv() {
        let mut wtr = csv::Writer::from_writer(vec![]);
        wtr.serialize(&test_data::<CsvSerialization>()).unwrap();

        let result = String::from_utf8(wtr.into_inner().unwrap()).unwrap();

        assert_eq!(
            result,
            r#"data
"a,b,c"
"#
        );
    }

    #[test]
    fn test_serialize_json() {
        let result = serde_json::to_string(&test_data::<JsonSerialization>()).unwrap();
        assert_eq!(result, r#"{"data":["a","b","c"]}"#);
    }
}
