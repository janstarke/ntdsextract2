use crate::RdnSet;

pub trait SerializationType {
    fn serialize<'a, S>(
        items: impl Iterator<Item = String>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer;

    fn deserialize<'de, D>(deserializer: D) -> Result<RdnSet<Self>, D::Error>
    where
        Self: Sized,
        D: serde::Deserializer<'de>;
}
