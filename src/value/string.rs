use crate::cache::Value;

use crate::ntds::Error;

use super::FromValue;

impl FromValue for String {
    fn from_value_opt(value: &Value) -> Result<Option<Self>, Error>
    where
        Self: Sized,
    {
        match value {
            Value::Text(val) => Ok(Some(val.as_ref().to_owned())),
            Value::LargeText(val) => Ok(Some(val.as_ref().to_owned())),
            Value::Binary(val) | Value::LargeBinary(val) => Ok(Some(hex::encode(val.as_ref()))),
            Value::Null(()) => Ok(None),
            _ => Err(Error::InvalidValueDetected(value.to_string(), "String (one of (text, largetext or binary)")),
        }
    }
}
