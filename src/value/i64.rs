use crate::cache::Value;

use crate::ntds::Error;

use super::FromValue;

impl FromValue for i64 {
    fn from_value_opt(value: &Value) -> Result<Option<Self>, Error>
    where
        Self: Sized,
    {
        match value {
            Value::I16(val) => Ok(Some(i64::from(*val))),
            Value::I32(val) => Ok(Some(i64::from(*val))),
            Value::I64(val) => Ok(Some(*val)),
            Value::Currency(val) => Ok(Some(*val)),
            Value::Binary(v) | Value::LargeBinary(v) if v.len() == 8 => {
                Ok(Some(i64::from_le_bytes([
                    v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7],
                ])))
            }
            Value::Null(()) => Ok(None),
            _ => panic!("test"), //Err(Error::InvalidValueDetected(value.to_string(), "i64")),
        }
    }
}
