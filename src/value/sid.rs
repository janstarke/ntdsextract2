use libesedb::Value;

use crate::win32_types::Sid;

use super::{ConversionError, FromValue};

impl FromValue for Sid {
    fn from_value_opt(value: &Value) -> Result<Option<Self>, ConversionError>
    where
        Self: Sized,
    {
        match value {
            Value::Binary(val) | Value::LargeBinary(val) => {
                Ok(Some(Sid::try_from(val).or_else(|why| {
                    Err(ConversionError::MiscConversionError {
                        value,
                        intended_type: "Sid",
                        why,
                    })
                })?))
            }
            Value::Null(()) => Ok(None),
            _ => Err(ConversionError::InvalidValueDetected(value)),
        }
    }
}
