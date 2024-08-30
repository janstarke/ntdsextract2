use std::{marker::PhantomData, sync::Mutex};

use serde::{Deserialize, Serialize};

use crate::{
    cache::RecordPointer,
    cli::MemberOfAttribute,
    object_tree::ObjectTree,
    win32_types::{Rdn, Sid},
    SerializationType,
};

static MEMBER_OF_ATTRIBUTE: Mutex<MemberOfAttribute> = Mutex::new(MemberOfAttribute::Rdn);

pub fn use_member_of_attribute(att: MemberOfAttribute) {
    *MEMBER_OF_ATTRIBUTE.lock().unwrap() = att;
}

pub fn member_of_attribute() -> MemberOfAttribute {
    *MEMBER_OF_ATTRIBUTE.lock().unwrap()
}

enum PointerOrString {
    Pointer(RecordPointer),
    String(String),

    // this can only be set when deserializing a value
    None,
}

pub struct Membership<T: SerializationType> {
    rdn: Rdn,
    sid: Option<Sid>,
    sam_account_name: Option<String>,
    dn: PointerOrString,
    phantom: PhantomData<T>,
}

impl<T> From<(RecordPointer, Rdn, Option<Sid>, Option<String>)> for Membership<T>
where
    T: SerializationType,
{
    fn from(value: (RecordPointer, Rdn, Option<Sid>, Option<String>)) -> Self {
        Self {
            rdn: value.1,
            sid: value.2,
            dn: PointerOrString::Pointer(value.0),
            sam_account_name: value.3,
            phantom: PhantomData,
        }
    }
}

impl<T> From<(String, Rdn, Option<Sid>, Option<String>)> for Membership<T>
where
    T: SerializationType,
{
    fn from(value: (String, Rdn, Option<Sid>, Option<String>)) -> Self {
        Self {
            rdn: value.1,
            sid: value.2,
            dn: PointerOrString::String(value.0),
            sam_account_name: value.3,
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
            dn: PointerOrString::None,
            sam_account_name: None,
            phantom: PhantomData,
        }
    }
}

impl<'de, T> Deserialize<'de> for Membership<T>
where
    T: SerializationType,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match Rdn::deserialize(deserializer) {
            Ok(rdn) => Ok(Self::from(rdn)),
            Err(why) => Err(why),
        }
    }
}

impl<T> Serialize for Membership<T>
where
    T: SerializationType,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match member_of_attribute() {
            MemberOfAttribute::Sid => {
                T::serialize(self.sid.as_ref().map(|s| s.to_string()), serializer)
            }
            MemberOfAttribute::Rdn => T::serialize(Some(self.rdn.to_string()), serializer),
            MemberOfAttribute::Dn => T::serialize(
                match &self.dn {
                    PointerOrString::Pointer(_ptr) => None,
                    PointerOrString::String(dn) => Some(dn.clone()),
                    PointerOrString::None => {
                        panic!("it is not expected to serialize a previously deserialized value")
                    }
                },
                serializer,
            ),
            MemberOfAttribute::SamAccountName => {
                T::serialize(self.sam_account_name.clone(), serializer)
            }
        }
    }
}

pub struct MembershipSet<T: SerializationType>(Vec<Membership<T>>);

impl<T: SerializationType> MembershipSet<T> {
    pub fn update_dn(&mut self, tree: &ObjectTree) {
        for m in self.0.iter_mut() {
            if let PointerOrString::Pointer(ptr) = m.dn {
                if let Some(dn) = tree.dn_of(&ptr) {
                    m.dn = PointerOrString::String(dn);
                }
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
            MemberOfAttribute::Sid => T::serialize_list(
                self.0.iter().map(|m| m.sid.as_ref().map(|s| s.to_string())),
                serializer,
            ),
            MemberOfAttribute::Rdn => {
                T::serialize_list(self.0.iter().map(|m| Some(m.rdn.to_string())), serializer)
            }
            MemberOfAttribute::Dn => T::serialize_list(
                self.0.iter().map(|m| match &m.dn {
                    PointerOrString::Pointer(ptr) => Some(format!("MISSING ENTRY FOR REFERENCE {ptr}")),
                    PointerOrString::String(dn) => Some(dn.clone()),
                    PointerOrString::None => {
                        panic!("it is not expected to serialize a previously deserialized value")
                    }
                }),
                serializer,
            ),
            MemberOfAttribute::SamAccountName => T::serialize_list(
                self.0.iter().map(|m| m.sam_account_name.clone()),
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
                        None,
                    )),
                    Membership::<T>::from((
                        RecordPointer::new(2.into(), 2.into()),
                        Rdn::try_from("b").unwrap(),
                        None,
                        None,
                    )),
                    Membership::<T>::from((
                        RecordPointer::new(3.into(), 3.into()),
                        Rdn::try_from("c").unwrap(),
                        None,
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
