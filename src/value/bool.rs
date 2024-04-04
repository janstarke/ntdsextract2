use crate::cache::Value;

use crate::ntds::Error;

use super::FromValue;

impl FromValue for bool {
    fn from_value_opt(value: &Value) -> Result<Option<Self>, Error>
    where
        Self: Sized,
    {
        match value {
            Value::Null(_) => Ok(None),
            Value::U8(val) => Ok(Some(*val == 1)),
            Value::U16(val) => Ok(Some(*val == 1)),
            Value::U32(val) => Ok(Some(*val == 1)),
            Value::I16(val) => Ok(Some(*val == 1)),
            Value::I32(val) => Ok(Some(*val == 1)),
            _ => Err(Error::InvalidValueDetected(value.to_string(), "bool (one of u8, u16, u32, i16 or i32)")),
        }
    }
}
