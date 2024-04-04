use crate::cache::Value;

use crate::ntds::Error;

use super::FromValue;

impl FromValue for i32 {
    fn from_value_opt(value: &Value) -> Result<Option<Self>, Error>
    where
        Self: Sized,
    {
        match value {
            Value::I32(val) => Ok(Some(*val)),
            Value::Null(()) => Ok(None),
            _ => Err(Error::InvalidValueDetected(value.to_string(), "i32")),
        }
    }
}
