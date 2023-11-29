use serde::ser::SerializeSeq;
use serde::Serializer;

pub enum SerializableSet {
    Flat(Vec<String>),
    Complex(Vec<String>),
}

pub(crate) fn serialize_set<S>(set: &SerializableSet, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match set {
        SerializableSet::Flat(v) => s.serialize_str(&v.join(",")),
        SerializableSet::Complex(v) => {
            let mut seq = s.serialize_seq(None)?;
            for o in v.iter() {
                seq.serialize_element(o)?;
            }
            seq.end()
        }
    }
}
