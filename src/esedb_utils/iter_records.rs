use std::iter::{Filter, Map};

use libesedb::Table;

use crate::column_info_mapping::DbRecord;

pub (crate) fn iter_records<'a, 'b>(data_table: &'b Table<'a>) -> Box<dyn Iterator<Item = DbRecord<'b>> + 'b>
where
    'a: 'b,
{
    Box::new(
        data_table
            .iter_records()
            .expect("unable to iterate this table")
            .filter_map(|r| r.ok())
            .map(DbRecord::from),
    )
}

pub (crate) fn filter_records_from<'a, 'b, P>(
    data_table: &'b Table<'a>,
    predicate: P,
) -> Box<dyn Iterator<Item = DbRecord<'b>> + 'b>
where
    P: FnMut(&DbRecord<'b>) -> bool + 'b,
{
    Box::new(iter_records(data_table)
        .filter(predicate)
        .map(|r| {log::trace!("found one object"); r}))
}

pub (crate) fn find_record_from<'a, 'b, P>(data_table: &'b Table<'a>, predicate: P) -> Option<DbRecord<'b>>
where
    P: FnMut(&DbRecord<'b>) -> bool,
{
    iter_records(data_table).find(predicate)
}
