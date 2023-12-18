use std::fmt::Display;

use getset::Getters;

use crate::value::FromValue;

use super::{EsedbRowId, RecordId};

#[derive(Getters, Hash, Debug, Clone, Copy)]
#[getset(get = "pub", set = "pub")]
pub struct RecordPointer {
    ds_record_id: RecordId,
    esedb_row: Option<EsedbRowId>,
}

impl RecordPointer {
    pub fn new(ds_record_id: RecordId, esedb_row: EsedbRowId) -> Self {
        Self {
            ds_record_id: ds_record_id,
            esedb_row: Some(esedb_row),
        }
    }
}

impl From<RecordId> for RecordPointer {
    fn from(ds_record_id: RecordId) -> Self {
        Self {
            ds_record_id: ds_record_id,
            esedb_row: None,
        }
    }
}

impl FromValue for RecordPointer {
    fn from_value_opt(value: &super::Value) -> crate::ntds::Result<Option<Self>>
    where
        Self: Sized,
    {
        Ok(RecordId::from_value_opt(value)?.map(Self::from))
    }
}

impl Display for RecordPointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.esedb_row {
            Some(row) => write!(f, "id={}/row={row}", self.ds_record_id),
            None => write!(f, "id={}", self.ds_record_id),
        }
    }
}

impl PartialEq for RecordPointer {
    fn eq(&self, other: &Self) -> bool {
        if self.ds_record_id == other.ds_record_id {
            true
        } else {
            self.esedb_row == other.esedb_row
        }
    }
}

impl Eq for RecordPointer {}
