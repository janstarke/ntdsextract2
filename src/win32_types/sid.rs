use std::{io::Cursor, fmt::Display};

use anyhow::{anyhow, ensure, Result};
use byteorder::{ReadBytesExt, BigEndian, LittleEndian};
use libesedb::Value;
use serde::Serialize;

use crate::esedb_utils::FromValue;

/// 
/// https://devblogs.microsoft.com/oldnewthing/20040315-00/?p=40253
#[derive(PartialEq)]
pub(crate) struct Sid {
  revision: u8,
  authority: u64,
  numbers: Vec<u32>,
}

impl Sid {
  pub fn get_rid(&self) -> &u32 {
    self.numbers.last().unwrap()
  }
}

impl TryFrom<Vec<u8>> for Sid {
    type Error = anyhow::Error;

    fn try_from(val: Vec<u8>) -> Result<Self, Self::Error> {
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
      ensure!(! numbers.is_empty(), "invalid SID format");

      Ok(Self {
        revision,
        authority,
        numbers
      })
    }
}

impl Display for Sid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let numbers = self.numbers
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
  fn from_value_opt(value: Value, attrib_name: &str) -> Result<Option<Sid>> {
      match value {
          Value::Binary(val) | Value::LargeBinary(val) => Ok(Some(Sid::try_from(val)?)),
          Value::Null => Ok(None),
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
        S: serde::Serializer {
        serializer.serialize_str(&format!("{self}"))
    }
}