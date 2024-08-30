use crate::MembershipSet;

pub trait SerializationType {
    fn serialize<S>(
        items: impl Iterator<Item = Option<String>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer;

    fn deserialize<'de, D>(deserializer: D) -> Result<MembershipSet<Self>, D::Error>
    where
        Self: Sized,
        D: serde::Deserializer<'de>;
}
