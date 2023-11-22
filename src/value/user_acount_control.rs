use libesedb::Value;

use crate::win32_types::UserAccountControl;

use super::{ConversionError, FromValue};

impl FromValue for UserAccountControl {
    fn from_value_opt(value: &Value) -> Result<Option<Self>, ConversionError>
    where
        Self: Sized,
    {
        match value {
            Value::I32(val) => Ok(Some(<UserAccountControl>::from_bits_truncate(
                u32::from_ne_bytes(val.to_ne_bytes()),
            ))),
            Value::Null(()) => Ok(None),
            _ => Err(ConversionError::InvalidValueDetected(value)),
        }
    }
}
