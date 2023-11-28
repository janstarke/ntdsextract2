use std::slice;

use crate::cache;

pub struct Iter<'info, 'db>(slice::Iter<'db, cache::Record<'info, 'db>>);

impl<'info, 'db> Iterator for Iter<'info, 'db>
{
    type Item = &'db cache::Record<'info, 'db>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'info, 'db> From<slice::Iter<'db, cache::Record<'info, 'db>>> for Iter<'info, 'db>
{
    fn from(value: slice::Iter<'db, cache::Record<'info, 'db>>) -> Self {
        Self(value)
    }
}