use libesedb::Value;

use crate::{ntds::Error, win32_types::Sid};

use super::FromValue;

impl FromValue for Sid {
    fn from_value_opt(value: &Value) -> Result<Option<Self>, Error>
    where
        Self: Sized,
    {
        match value {
            Value::Binary(val) | Value::LargeBinary(val) => {
                Ok(Some(Sid::try_from(val).or_else(|why| {
                    Err(Error::MiscConversionError {
                        value: value.to_string(),
                        intended_type: "Sid",
                        why,
                    })
                })?))
            }
            Value::Null(()) => Ok(None),
            _ => Err(Error::InvalidValueDetected(value.to_string())),
        }
    }
}
