use anyhow::{anyhow, Result};
use libesedb::Value;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde::{Serialize, Deserialize};
use strum::EnumString;

use crate::esedb_utils::FromValue;

#[derive(EnumString, FromPrimitive, Deserialize, Serialize, PartialEq)]
#[allow(non_camel_case_types)]
pub enum SamAccountType {
    SAM_GROUP_OBJECT = 0x10000000,
    SAM_NON_SECURITY_GROUP_OBJECT = 0x10000001,
    SAM_ALIAS_OBJECT = 0x20000000,
    SAM_NON_SECURITY_ALIAS_OBJECT = 0x20000001,
    SAM_USER_OBJECT = 0x30000000,
    SAM_MACHINE_ACCOUNT = 0x30000001,
    SAM_TRUST_ACCOUNT = 0x30000002,
    SAM_APP_BASIC_GROUP = 0x40000000,
    SAM_APP_QUERY_GROUP = 0x40000001,
}

impl FromValue for SamAccountType {
    fn from_value_opt(value: &Value, attrib_name: &str) -> Result<Option<SamAccountType>> {
        match value {
            Value::I32(val) => Ok(FromPrimitive::from_u32(u32::from_ne_bytes(
                val.to_ne_bytes(),
            ))),
            Value::Null(()) => Ok(None),
            _ => Err(anyhow!(
                "invalid value detected: {:?} in field {}",
                value,
                attrib_name
            )),
        }
    }
}
