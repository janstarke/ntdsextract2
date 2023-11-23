use libesedb::Value;
use crate::ntds::{Result, Error};

pub trait FromValue<'a> {
    /// does the same as [`from_value_opt`], but returns an Error instead of `None`, if no value was found
    fn from_value(value: &'a Value) -> Result<'a, Self>
    where
        Self: Sized,
    {
        Self::from_value_opt(value)?.ok_or(Error::ValueIsMissing)
    }

    /// converts the value into the requested type, if possible
    fn from_value_opt(value: &'a Value) -> Result<'a, Option<Self>>
    where
        Self: Sized;
}
