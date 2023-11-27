use std::slice;

use crate::cache;
use crate::cache::EsedbRecord;

use super::RecordIterator;


pub struct Iter<'table, 'record>(slice::Iter<'table, cache::Record<'record>>);

impl<'table, 'record> Iterator for Iter<'table, 'record>
{
    type Item = &'table cache::Record<'record>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'table, 'record> RecordIterator<'table, 'record> for Iter<'table, 'record> {}

impl<'table, 'record> From<slice::Iter<'table, cache::Record<'record>>> for Iter<'table, 'record>
{
    fn from(value: slice::Iter<'table, cache::Record<'record>>) -> Self {
        Self(value)
    }
}