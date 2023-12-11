use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::SerializationType;

pub struct StringSet<T: SerializationType>(Vec<String>, PhantomData<T>);

impl<'a, T> From<Vec<&'a str>> for StringSet<T>
where
    T: SerializationType,
{
    fn from(value: Vec<&'a str>) -> Self {
        Self(
            value.into_iter().map(|s| s.to_owned()).collect(),
            PhantomData,
        )
    }
}

impl<T> From<Vec<String>> for StringSet<T>
where
    T: SerializationType,
{
    fn from(value: Vec<String>) -> Self {
        Self(value, PhantomData)
    }
}

impl<T> Serialize for StringSet<T>
where
    T: SerializationType,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        T::serialize(self.0.iter().map(|s| &s[..]), serializer)
    }
}

impl<'de, T> Deserialize<'de> for StringSet<T>
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

    use crate::{CsvSerialization, JsonSerialization};

    use super::{SerializationType, StringSet};

    #[derive(Serialize)]
    #[serde(bound = "T: SerializationType")]
    struct SampleRecord<T: SerializationType> {
        data: StringSet<T>,
    }

    fn test_data<T>() -> SampleRecord<T>
    where
        T: SerializationType,
    {
        SampleRecord {
            data: StringSet::<T>::from(vec!["a", "b", "c"]),
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
