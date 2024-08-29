use crate::{cache::ColumnIndex, EsedbInfo};

use super::NtdsAttributeId;

impl NtdsAttributeId {
    pub fn id<'info>(&self, info: &'info EsedbInfo) -> &'info ColumnIndex {
        info.column(*self).id()
    }
}