use std::fmt::Display;
use core::hash::Hash;

use getset::Getters;

use super::{EsedbRowId, RecordId};

#[derive(Getters, Debug, Clone, Copy)]
#[getset(get = "pub", set = "pub")]
pub struct RecordPointer {
    ds_record_id: RecordId,
    esedb_row: EsedbRowId,
}

impl RecordPointer {
    pub fn new(ds_record_id: RecordId, esedb_row: EsedbRowId) -> Self {
        Self {
            ds_record_id,
            esedb_row,
        }
    }
}

impl Display for RecordPointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id={}/row={}", self.ds_record_id, self.esedb_row)
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

impl Hash for RecordPointer {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ds_record_id.hash(state);
        self.esedb_row.hash(state);
    }
}
