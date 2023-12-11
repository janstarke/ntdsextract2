use std::marker::PhantomData;

use serde::ser::SerializeSeq;
use serde::Serialize;

pub struct StringSet<T: SerializationType>(Vec<String>, PhantomData<T>);

impl<'a, T> From<Vec<&'a str>> for StringSet<T> where 
    T: SerializationType,
{
    fn from(value: Vec<&'a str>) -> Self {
        Self(value.into_iter().map(|s| s.to_owned()).collect(), PhantomData)
    }
}

impl<T> From<Vec<String>> for StringSet<T>
where T: SerializationType {
    fn from(value: Vec<String>) -> Self {
        Self (
            value,
            PhantomData
        )
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

pub trait SerializationType {
    fn serialize<'a, S>(
        items: impl Iterator<Item = &'a str>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer;
}
pub struct CsvSerialization;

pub struct JsonSerialization;

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
}
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
}

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use super::{CsvSerialization, JsonSerialization, SerializationType, StringSet};

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
