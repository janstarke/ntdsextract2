use libesedb::Value;

use crate::ntds::Error;

use super::FromValue;

impl<'a> FromValue<'a> for u32 {
    fn from_value_opt(value: &Value) -> Result<Option<Self>, Error>
    where
        Self: Sized,
    {
        match value {
            Value::U8(val) => Ok(Some((*val).into())),
            Value::U16(val) => Ok(Some((*val).into())),
            Value::U32(val) => Ok(Some(*val)),
            Value::I16(val) => Ok(Some((*val).try_into().or_else(|why| {
                Err(Error::IntegerConversionError {
                    value: value.to_string(),
                    intended_type: "i16",
                    why,
                })
            })?)),
            Value::I32(val) => Ok(Some((*val).try_into().or_else(|why| {
                Err(Error::IntegerConversionError {
                    value: value.to_string(),
                    intended_type: "i16",
                    why,
                })
            })?)),
            Value::Null(()) => Ok(None),
            _ => Err(Error::InvalidValueDetected(value.to_string())),
        }
    }
}
