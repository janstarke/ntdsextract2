use serde::{Deserialize, Serialize};

use crate::ntds::DataTableRecord;

use super::HasSerializableFields;


pub trait SpecificObjectAttributes: for<'de> Deserialize<'de> + Serialize + HasSerializableFields {
    fn from(record: &DataTableRecord) -> anyhow::Result<Self>;
    fn serialize_to<S>(&self, s: &mut S::SerializeStruct) -> Result<(), S::Error> where S: serde::Serializer;
}
