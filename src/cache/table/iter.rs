use std::slice;

use crate::{cache, ntds::DataTableRecord};

pub struct Iter<'info, 'db>(slice::Iter<'db, cache::Record<'info, 'db>>);

impl<'info, 'db> From<slice::Iter<'db, cache::Record<'info, 'db>>> for Iter<'info, 'db>
{
    fn from(value: slice::Iter<'db, cache::Record<'info, 'db>>) -> Self {
        Self(value)
    }
}

impl<'info, 'db> Iterator for Iter<'info, 'db>
{
    type Item = DataTableRecord<'info, 'db>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(DataTableRecord::from)
    }
}