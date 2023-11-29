use crate::{cache, RecordHasAttRdn, RecordHasId};
use crate::{ntds::DataTableRecord, RecordHasParent, RecordPredicate};

use super::Iter;
pub type DataTable<'info, 'db> = cache::Table<'info, 'db>;

pub struct Record<'info, 'db>(Iter<'info, 'db>);

impl<'info, 'db> Iterator for DataTableIter<'info, 'db> {
    type Item = DataTableRecord<'info, 'db>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(DataTableRecord::from)
    }
}

impl<'info, 'db> From<cache::Iter<'info, 'db>>
    for DataTableIter<'info, 'db>
{
    fn from(value: cache::Iter<'info, 'db>) -> Self {
        Self(value)
    }
}
