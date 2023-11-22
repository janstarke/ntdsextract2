use libesedb::Value;

use super::{ConversionError, FromValue};

impl FromValue for i32 {
    fn from_value_opt(value: &Value) -> Result<Option<Self>, ConversionError>
    where
        Self: Sized,
    {
        match value {
            Value::I32(val) => Ok(Some(*val)),
            Value::Null(()) => Ok(None),
            _ => Err(ConversionError::InvalidValueDetected(value)),
        }
    }
}
