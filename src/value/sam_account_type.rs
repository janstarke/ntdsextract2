use libesedb::Value;
use num_traits::FromPrimitive;

use crate::{ntds::Error, win32_types::SamAccountType};

use super::FromValue;

impl FromValue for SamAccountType {
    fn from_value_opt(value: &Value) -> Result<Option<Self>, Error> {
        match value {
            Value::I32(val) => Ok(FromPrimitive::from_u32(u32::from_ne_bytes(
                val.to_ne_bytes(),
            ))),
            Value::Null(()) => Ok(None),
            _ => Err(Error::InvalidValueDetected(value.to_string())),
        }
    }
}
