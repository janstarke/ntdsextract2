use crate::cache;

pub trait RecordIterator<'s, 'record: 's>: Iterator<Item = &'s cache::Record<'record>>
{
}

