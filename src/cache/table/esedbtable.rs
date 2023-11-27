use crate::cache::EsedbRecord;

use crate::cache::table::Iter;

pub trait EsedbTable<'table, 'record>
{
    fn iter<'s>(&'s self) -> Iter<'s, 'record>;
}
