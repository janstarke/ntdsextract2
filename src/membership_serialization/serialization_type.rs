use crate::MembershipSet;

pub trait SerializationType {
    fn serialize_list<S>(
        items: impl Iterator<Item = Option<String>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer;

    fn serialize<S>(item: Option<String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match item {
            Some(v) => serializer.serialize_str(&v),
            None => serializer.serialize_none(),
        }
    }

    fn deserialize<'de, D>(deserializer: D) -> Result<MembershipSet<Self>, D::Error>
    where
        Self: Sized,
        D: serde::Deserializer<'de>;
}
