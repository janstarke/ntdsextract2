use libesedb::Value;

use super::{ConversionError, FromValue};

impl FromValue for String {
    fn from_value_opt(value: &Value) -> Result<Option<Self>, ConversionError>
    where
        Self: Sized,
    {
        match value {
            Value::Text(val) => Ok(Some(val.to_owned())),
            Value::LargeText(val) => Ok(Some(val.to_owned())),
            Value::Binary(val) | Value::LargeBinary(val) => Ok(Some(hex::encode(val))),
            Value::Null(()) => Ok(None),
            _ => Err(ConversionError::InvalidValueDetected(value)),
        }
    }
}

impl FromValue for &str {
    fn from_value_opt(value: &Value) -> Result<Option<Self>, ConversionError>
    where
        Self: Sized,
    {
        match value {
            Value::Text(val) => Ok(Some(&val[..])),
            Value::LargeText(val) => Ok(Some(&val[..])),
            Value::Null(()) => Ok(None),
            _ => Err(ConversionError::InvalidValueDetected(value)),
        }
    }
}
