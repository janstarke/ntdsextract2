use std::{marker::PhantomData, sync::Mutex};

use serde::{Deserialize, Serialize};

use crate::{
    cache::RecordPointer, cli::MemberOfAttribute, object_tree::ObjectTree, win32_types::{Rdn, Sid}, SerializationType
};

static MEMBER_OF_ATTRIBUTE: Mutex<MemberOfAttribute> = Mutex::new(MemberOfAttribute::Rdn);

pub fn use_member_of_attribute(att: MemberOfAttribute) {
    *MEMBER_OF_ATTRIBUTE.lock().unwrap() = att;
}

pub fn member_of_attribute() -> MemberOfAttribute {
    *MEMBER_OF_ATTRIBUTE.lock().unwrap()
}

enum PointerOrDn {
    Pointer(RecordPointer),
    Dn(String),

    // this can only be set when deserializing a value
    None,
}

pub struct Membership<T: SerializationType> {
    rdn: Rdn,
    sid: Option<Sid>,
    dn: PointerOrDn,
    phantom: PhantomData<T>,
}

impl<T> From<(RecordPointer, Rdn, Option<Sid>)> for Membership<T>
where
    T: SerializationType,
{
    fn from(value: (RecordPointer, Rdn, Option<Sid>)) -> Self {
        Self {
            rdn: value.1,
            sid: value.2,
            dn: PointerOrDn::Pointer(value.0),
            phantom: PhantomData,
        }
    }
}

impl<T> From<Rdn> for Membership<T>
where
    T: SerializationType,
{
    fn from(rdn: Rdn) -> Self {
        Self {
            rdn,
            sid: None,
            dn: PointerOrDn::None,
            phantom: PhantomData,
        }
    }
}

pub struct MembershipSet<T: SerializationType>(Vec<Membership<T>>);

impl<T: SerializationType> MembershipSet<T> {
    pub fn update_dn(&mut self, tree: &ObjectTree) {
        for m in self.0.iter_mut() {
            if let PointerOrDn::Pointer(ptr) = m.dn {
                let dn = tree.dn_of(&ptr);
                m.dn = PointerOrDn::Dn(dn);
            }
        }
    }
}

impl<T, I> From<I> for MembershipSet<T>
where
    I: Iterator<Item = Membership<T>>,
    T: SerializationType,
{
    fn from(iter: I) -> Self {
        Self(iter.collect())
    }
}

impl<T> Serialize for MembershipSet<T>
where
    T: SerializationType,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match member_of_attribute() {
            MemberOfAttribute::Sid => T::serialize(
                self.0
                    .iter()
                    .map(|m| m.sid.as_ref().map(|s| s.to_string())),
                serializer,
            ),
            MemberOfAttribute::Rdn => {
                T::serialize(self.0.iter().map(|m| Some(m.rdn.to_string())), serializer)
            }
            MemberOfAttribute::Dn => T::serialize(
                self.0.iter().map(|m| match &m.dn {
                    PointerOrDn::Pointer(ptr) => Some(ptr.to_string()),
                    PointerOrDn::Dn(dn) => Some(dn.clone()),
                    PointerOrDn::None => {
                        panic!("it is not expected to serialize a previously deserialized value")
                    }
                }),
                serializer,
            ),
        }
    }
}

impl<'de, T> Deserialize<'de> for MembershipSet<T>
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

    use crate::{
        cache::RecordPointer, cli::MemberOfAttribute, use_member_of_attribute, win32_types::Rdn,
        CsvSerialization, JsonSerialization,
    };

    use super::{Membership, MembershipSet, SerializationType};

    #[derive(Serialize)]
    #[serde(bound = "T: SerializationType")]
    struct SampleRecord<T: SerializationType> {
        data: MembershipSet<T>,
    }

    fn test_data<T>() -> SampleRecord<T>
    where
        T: SerializationType,
    {
        SampleRecord {
            data: MembershipSet::<T>::from(
                vec![
                    Membership::<T>::from((
                        RecordPointer::new(1.into(), 1.into()),
                        Rdn::try_from("a").unwrap(),
                        None,
                    )),
                    Membership::<T>::from((
                        RecordPointer::new(2.into(), 2.into()),
                        Rdn::try_from("b").unwrap(),
                        None,
                    )),
                    Membership::<T>::from((
                        RecordPointer::new(3.into(), 3.into()),
                        Rdn::try_from("c").unwrap(),
                        None,
                    )),
                ]
                .into_iter(),
            ),
        }
    }

    #[test]
    fn test_serialize_csv() {
        use_member_of_attribute(MemberOfAttribute::Rdn);

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
        use_member_of_attribute(MemberOfAttribute::Rdn);
        let result = serde_json::to_string(&test_data::<JsonSerialization>()).unwrap();
        assert_eq!(result, r#"{"data":["a","b","c"]}"#);
    }
}
