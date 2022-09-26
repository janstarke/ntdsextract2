use libesedb::Table;

use crate::column_info_mapping::{ColumnInfoMapping, DbRecord, value_to_rid};

pub(crate) fn iter_records<'a, 'b>(
    data_table: &'b Table<'a>,
) -> Box<dyn Iterator<Item = DbRecord<'b>> + 'b>
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

pub(crate) fn filter_records_from<'a, 'b, P>(
    data_table: &'b Table<'a>,
    predicate: P,
) -> Box<dyn Iterator<Item = DbRecord<'b>> + 'b>
where
    P: FnMut(&DbRecord<'b>) -> bool + 'b,
    'a: 'b,
{
    Box::new(iter_records(data_table).filter(predicate).map(|r| {
        log::trace!("found one object");
        r
    }))
}

pub(crate) fn find_record_from<'a, 'b, P>(
    data_table: &'b Table<'a>,
    predicate: P,
) -> Option<DbRecord<'b>>
where
    P: FnMut(&DbRecord<'b>) -> bool,
    'a: 'b,
{
    iter_records(data_table).find(predicate)
}

pub(crate) fn find_by_id<'a, 'b>(
    table: &'b Table<'a>,
    mapping: &ColumnInfoMapping,
    index: i32,
) -> Option<DbRecord<'b>>
where
    'a: 'b,
{
    find_record_from(table, |d| {
        d.ds_record_id(mapping)
            .expect("unable to read object record id")
            .expect("object has no record id")
            == index
    })
}

pub(crate) fn find_by_rid<'a, 'b>(
    table: &'b Table<'a>,
    mapping: &ColumnInfoMapping,
    object_rid: u32,
) -> Option<DbRecord<'b>>
where
    'a: 'b,
{
    find_record_from(table, |d| {
        d.value_of_ds_sid(mapping)
            .and_then(|sid| {
                value_to_rid(sid, "ATTj589922").ok().flatten().map(|rid| rid == object_rid)
            }).unwrap_or(false)
    })
}
