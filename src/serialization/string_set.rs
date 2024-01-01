use std::marker::PhantomData;

use serde::{ser::SerializeSeq, Deserialize, Serialize};

use crate::{win32_types::NameWithGuid, SerializationType};

pub struct StringSet<T: SerializationType>(Vec<NameWithGuid>, PhantomData<T>);

impl<'a, T> From<Vec<&'a NameWithGuid>> for StringSet<T>
where
    T: SerializationType,
{
    fn from(value: Vec<&'a NameWithGuid>) -> Self {
        Self(value.into_iter().cloned().collect(), PhantomData)
    }
}

impl<T> From<Vec<NameWithGuid>> for StringSet<T>
where
    T: SerializationType,
{
    fn from(value: Vec<NameWithGuid>) -> Self {
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
        let mut ser = serializer.serialize_seq(Some(self.0.len()))?;
        for s in self.0.iter() {
            ser.serialize_element(s)?;
        }
        ser.end()
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

    use crate::{win32_types::NameWithGuid, CsvSerialization, JsonSerialization};

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
            data: StringSet::<T>::from(vec![
                NameWithGuid::try_from("a").unwrap(),
                NameWithGuid::try_from("b").unwrap(),
                NameWithGuid::try_from("c").unwrap(),
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
