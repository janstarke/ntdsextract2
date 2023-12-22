use std::fmt::Display;

use super::RecordPointer;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Default, Debug)]
pub struct EsedbRowId(i32);

impl From<i32> for EsedbRowId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl Display for EsedbRowId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl EsedbRowId {
    pub fn inner(&self) -> i32 {
        self.0
    }

    pub fn step(&mut self) {
        self.0 += 1;
    }
}

impl From<RecordPointer> for EsedbRowId {
    fn from(value: RecordPointer) -> Self {
        *value.esedb_row()
    }
}