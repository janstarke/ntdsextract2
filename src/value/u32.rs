use libesedb::Value;

use super::{ConversionError, FromValue};

impl FromValue for u32 {
    fn from_value_opt(value: &Value) -> Result<Option<Self>, ConversionError>
    where
        Self: Sized,
    {
        match value {
            Value::U8(val) => Ok(Some((*val).into())),
            Value::U16(val) => Ok(Some((*val).into())),
            Value::U32(val) => Ok(Some(*val)),
            Value::I16(val) => Ok(Some((*val).try_into().or_else(|why| {
                Err(ConversionError::IntegerConversionError {
                    value,
                    intended_type: "i16",
                    why,
                })
            })?)),
            Value::I32(val) => Ok(Some((*val).try_into().or_else(|why| {
                Err(ConversionError::IntegerConversionError {
                    value,
                    intended_type: "i16",
                    why,
                })
            })?)),
            Value::Null(()) => Ok(None),
            _ => Err(ConversionError::InvalidValueDetected(value)),
        }
    }
}
