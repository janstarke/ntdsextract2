use libesedb::Value;

use crate::ntds::Error;

use super::FromValue;

impl<'a> FromValue<'a> for String {
    fn from_value_opt(value: &Value) -> Result<Option<Self>, Error>
    where
        Self: Sized,
    {
        match value {
            Value::Text(val) => Ok(Some(val.to_owned())),
            Value::LargeText(val) => Ok(Some(val.to_owned())),
            Value::Binary(val) | Value::LargeBinary(val) => Ok(Some(hex::encode(val))),
            Value::Null(()) => Ok(None),
            _ => Err(Error::InvalidValueDetected(value.to_string())),
        }
    }
}

impl<'a> FromValue<'a> for &'a str {
    fn from_value_opt(value: &'a Value) -> Result<Option<Self>, Error>
    where
        Self: Sized,
    {
        match value {
            Value::Text(val) => Ok(Some(&val[..])),
            Value::LargeText(val) => Ok(Some(&val[..])),
            Value::Null(()) => Ok(None),
            _ => Err(Error::InvalidValueDetected(value.to_string())),
        }
    }
}
