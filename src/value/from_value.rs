use super::{ConversionError, ConversionResult};
use libesedb::Value;

pub trait FromValue {
    /// does the same as [`from_value_opt`], but returns an Error instead of `None`, if no value was found
    fn from_value(value: &Value) -> ConversionResult<Self>
    where
        Self: Sized,
    {
        Self::from_value_opt(value)?.ok_or(ConversionError::ValueIsMissing)
    }

    /// converts the value into the requested type, if possible
    fn from_value_opt(value: &Value) -> ConversionResult<Option<Self>>
    where
        Self: Sized;
}
