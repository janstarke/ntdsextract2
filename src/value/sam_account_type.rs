use libesedb::Value;
use num_traits::FromPrimitive;

use crate::win32_types::SamAccountType;

use super::{ConversionError, FromValue};

impl FromValue for SamAccountType {
    fn from_value_opt(value: &Value) -> Result<Option<Self>, ConversionError> {
        match value {
            Value::I32(val) => Ok(FromPrimitive::from_u32(u32::from_ne_bytes(
                val.to_ne_bytes(),
            ))),
            Value::Null(()) => Ok(None),
            _ => Err(ConversionError::InvalidValueDetected(value)),
        }
    }
}
