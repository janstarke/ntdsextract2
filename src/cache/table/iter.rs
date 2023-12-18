use std::slice;

use crate::{cache::{self, EsedbRowId}, ntds::DataTableRecord};

pub struct Iter<'info, 'db>{
    inner: slice::Iter<'db, cache::Record<'info, 'db>>,
    row: EsedbRowId
}

impl<'info, 'db> From<slice::Iter<'db, cache::Record<'info, 'db>>> for Iter<'info, 'db>
{
    fn from(inner: slice::Iter<'db, cache::Record<'info, 'db>>) -> Self {
        Self{
            inner,
            row: Default::default()
        }
    }
}

impl<'info, 'db> Iterator for Iter<'info, 'db>
{
    type Item = DataTableRecord<'info, 'db>;

    fn next(&mut self) -> Option<Self::Item> {
        let row = self.row;
        self.row.step();
        self.inner.next().map(|r| DataTableRecord::new(r, row))
    }
}