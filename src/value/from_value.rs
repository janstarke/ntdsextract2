use libesedb::Value;
use crate::ntds::{Result, Error};

pub trait FromValue {
    /// does the same as [`from_value_opt`], but returns an Error instead of `None`, if no value was found
    fn from_value(value: &Value) -> Result<Self>
    where
        Self: Sized,
    {
        Self::from_value_opt(value)?.ok_or(Error::ValueIsMissing)
    }

    /// converts the value into the requested type, if possible
    fn from_value_opt(value: &Value) -> Result<Option<Self>>
    where
        Self: Sized;
}
