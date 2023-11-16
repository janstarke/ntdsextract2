use std::{fmt::Display, io::Cursor};

use anyhow::{anyhow, ensure, Result};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use libesedb::Value;
use serde::{Deserialize, Serialize};

use crate::esedb_utils::FromValue;
mod sid_visitor;

///
/// https://devblogs.microsoft.com/oldnewthing/20040315-00/?p=40253
#[derive(PartialEq)]
pub struct Sid {
    revision: u8,
    authority: u64,
    numbers: Vec<u32>,
}

impl Sid {
    pub fn get_rid(&self) -> &u32 {
        self.numbers.last().unwrap()
    }
    pub fn new(revision: u8, authority: u64, numbers: Vec<u32>) -> Self {
        Self {
            revision,
            authority,
            numbers,
        }
    }
}

impl TryFrom<&Vec<u8>> for Sid {
    type Error = anyhow::Error;

    fn try_from(val: &Vec<u8>) -> Result<Self, Self::Error> {
        let mut rdr = Cursor::new(val);
        let revision = rdr.read_u8()?;
        let number_of_dashes = rdr.read_u8()?;
        let authority = rdr.read_u48::<BigEndian>()?;

        //log::debug!("authority: {:012x}", authority);

        let mut numbers = vec![];
        for _i in 0..number_of_dashes - 1 {
            numbers.push(rdr.read_u32::<LittleEndian>()?);
        }
        numbers.push(rdr.read_u32::<BigEndian>()?);
        ensure!(!numbers.is_empty(), "invalid SID format");

        Ok(Self {
            revision,
            authority,
            numbers,
        })
    }
}

impl Display for Sid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let numbers = self
            .numbers
            .iter()
            .map(|n| format!("{n}"))
            .collect::<Vec<String>>()
            .join("-");

        let revision = &self.revision;
        let authority = &self.authority;
        write!(f, "S-{revision}-{authority}-{numbers}")
    }
}

impl FromValue for Sid {
    fn from_value_opt(value: &Value, attrib_name: &str) -> Result<Option<Sid>> {
        match value {
            Value::Binary(val) | Value::LargeBinary(val) => Ok(Some(Sid::try_from(val)?)),
            Value::Null(()) => Ok(None),
            _ => Err(anyhow!(
                "invalid value detected: {:?} in field {}",
                value,
                attrib_name
            )),
        }
    }
}

impl Serialize for Sid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{self}"))
    }
}

impl<'de> Deserialize<'de> for Sid {
    fn deserialize<D>(deserializer: D) -> std::prelude::v1::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(sid_visitor::SIDVisitor::default())
    }
}


#[cfg(test)]
mod tests {
    use super::Sid;

    #[test]
    fn test_deserialization() {
        let sample = r#""S-1-5-21-2623811015-3361044348-030300820-1013""#;
        let sid: Sid = serde_json::from_str(sample).unwrap();
        assert_eq!(sid.revision, 1);
        assert_eq!(sid.authority, 5);
        assert_eq!(sid.numbers, vec![21, 2_623_811_015, 3_361_044_348, 30_300_820, 1013]);
    }
}