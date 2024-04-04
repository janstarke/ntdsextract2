use std::{str::FromStr, fmt::Display};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{cache::Value, value::FromValue};

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Hash)]
pub struct Guid(Uuid);

impl FromValue for Guid {
    fn from_value_opt(value: &crate::cache::Value) -> crate::ntds::Result<Option<Self>>
    where
        Self: Sized,
    {
        match value {
            Value::Null(_) => Ok(None),
            Value::Binary(v) | Value::LargeBinary(v) | Value::Guid(v) => {
                Ok(Some(Self(Uuid::from_slice_le(&v[..])?)))
            }
            v => {
                log::error!("I don't know how to extract GUIDs from {v}");
                Ok(None)
            }
        }
    }
}

impl FromStr for Guid {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::from_str(s)?))
    }
}

impl Display for Guid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
