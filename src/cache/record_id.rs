use std::fmt::Display;

use crate::value::FromValue;

use super::RecordPointer;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, Default)]
pub struct RecordId(i32);

impl From<i32> for RecordId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl Display for RecordId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl FromValue for RecordId {
    fn from_value_opt(value: &super::Value) -> crate::ntds::Result<Option<Self>>
    where
        Self: Sized {
        Ok(i32::from_value_opt(value)?.map(Self::from))
    }
}

impl RecordId {
    pub fn inner(&self) -> i32 {
        self.0
    }
}

impl From<RecordPointer> for RecordId {
    fn from(value: RecordPointer) -> Self {
        *value.ds_record_id()
    }
}