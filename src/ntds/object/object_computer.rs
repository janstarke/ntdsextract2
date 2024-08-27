use lazy_static::lazy_static;
use serde::{ser::SerializeStruct, Deserialize, Serialize};

use crate::win32_types::Sid;

use crate::ntds::{types, HasSerializableFields, Object};

use super::SpecificObjectAttributes;

#[derive(Deserialize, Serialize)]
pub struct SpecificComputerAttributes {
    creator_sid: Option<Sid>,
}

impl HasSerializableFields for SpecificComputerAttributes {
    fn fields() -> &'static Vec<&'static str> {
        lazy_static! {
            static ref COMPUTER_HEADER: Vec<&'static str> = vec!["creator_sid"];
        }
        &COMPUTER_HEADER
    }
}

impl SpecificObjectAttributes for SpecificComputerAttributes {
    fn from(record: &crate::ntds::DataTableRecord) -> anyhow::Result<Self> {
        let creator_sid = record.att_creator_sid_opt()?;
        Ok(Self { creator_sid })
    }

    fn serialize_to<S>(&self, s: &mut S::SerializeStruct) -> Result<(), S::Error>
    where
        S: serde::Serializer,
    {
        s.serialize_field("creator_sid", &self.creator_sid)?;
        Ok(())
    }
}

pub type Computer<T> = Object<T, types::Computer, SpecificComputerAttributes>;
