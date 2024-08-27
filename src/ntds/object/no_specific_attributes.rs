use serde::{Deserialize, Serialize};

use crate::ntds::DataTableRecord;

use super::{HasSerializableFields, SpecificObjectAttributes};

pub struct NoSpecificAttributes;

impl HasSerializableFields for NoSpecificAttributes {
    fn fields() -> &'static Vec<&'static str> {
        static NO_HEADER: Vec<&'static str> = vec![];
        &NO_HEADER
    }
}

impl SpecificObjectAttributes for NoSpecificAttributes {
    fn from(_record: &DataTableRecord) -> anyhow::Result<Self> {
        Ok(Self)
    }

    fn serialize_to<S>(&self, _s: &mut S::SerializeStruct) -> Result<(), S::Error> where S: serde::Serializer {
        Ok(())
    }
}

impl Serialize for NoSpecificAttributes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_none()
    }
}

impl<'de> Deserialize<'de> for NoSpecificAttributes {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        Ok(Self)
    }
}