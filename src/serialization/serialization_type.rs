use crate::StringSet;


pub trait SerializationType {
    fn serialize<'a, S>(
        items: impl Iterator<Item = &'a str>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer;

    fn deserialize<'de, D>(deserializer: D) -> Result<StringSet<Self>, D::Error>
    where
        Self: Sized,
        D: serde::Deserializer<'de>;
}