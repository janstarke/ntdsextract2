use crate::{cache, RecordHasAttRdn, RecordHasId};
use crate::{ntds::DataTableRecord, RecordHasParent, RecordPredicate};

use super::{EsedbRecord, EsedbTable, Iter};

trait DataTableTrait<'table, R>
where
    for<'record> R: EsedbRecord<'record>,
{
}
pub type DataTable<'table, 'record> = cache::Table<'table, 'record>;

impl<'table, R, CDT> DataTableTrait<'table, R> for CDT
where
    CDT: DataTableTrait<'table, R>,
    R: for<'record> EsedbRecord<'record>,
{
}

trait DataTableIterator<'table, 'record: 'table>: Iterator<Item = &'table DataTableRecord<'table, 'record>>
{
}

pub struct DataTableIter<'table, 'record>(Iter<'table, 'record>);

impl<'table, 'record> Iterator for DataTableIter<'table, 'record>
{
    type Item = DataTableRecord<'table, 'record>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(DataTableRecord::from)
    }
}

impl<'table, 'record: 'table> From<cache::Iter<'table, 'record>> for DataTableIter<'table, 'record>
{
    fn from(value: cache::Iter<'table, 'record>) -> Self {
        Self(value)
    }
}

impl<'table, 'record: 'table> DataTable<'table, 'record>
{
    pub fn iter_records<'this: 'table>(&'this self) -> impl Iterator<Item = DataTableRecord<'table, 'record>>
    {
        DataTableIter::from(self.iter())
    }

    pub(crate) fn children_of<'this: 'table>(&'this self, parent_id: i32) -> impl Iterator<Item=DataTableRecord<'table, 'record>>
    {
        let my_filter = RecordHasParent(parent_id);
        self.iter_records().filter(move |r| my_filter.matches(r))
    }

    pub fn filter<'this: 'table, C>(&'this self, predicate: C) -> impl Iterator<Item=DataTableRecord<'table, 'record>> where
        C: Fn(&DataTableRecord<'table, 'record>) -> bool,
    {
        self.iter_records().filter(move |r| predicate(r))
    }

    pub fn find<'this: 'table, C>(&'this self, predicate: C) -> Option<DataTableRecord<'table, 'record>>
    where
        C: Fn(&DataTableRecord<'table, 'record>) -> bool,
    {
        self.iter_records().find(move |r| predicate(r))
    }

    pub fn filter_p<'this: 'table, P>(&'this self, predicate: P) -> impl Iterator<Item=DataTableRecord<'table, 'record>>
    where
        P: RecordPredicate<'table, 'record>,
    {
        self.iter_records().filter(move |r| predicate.matches(r))
    }

    pub fn find_p<'this: 'table, P>(&'this self, predicate: P) -> Option<DataTableRecord<'table, 'record>>
    where
        P: RecordPredicate<'table, 'record>,
    {
        self.iter_records().find(move |r| predicate.matches(r))
    }

    /// returns the record id of the record which contains the Schema object
    /// (which is identified by its name "Schema" in the object_name2 attribute)
    pub fn get_schema_record_id(&self) -> crate::ntds::Result<i32> {
        log::info!("obtaining schema record id");

        for record in self
            .filter_p(RecordHasAttRdn("Schema"))
            .map(DataTableRecord::from)
        {
            if let Some(schema_parent_id) = record.ds_parent_record_id_opt()? {
                if let Some(schema_parent) = self.find_p(RecordHasId(schema_parent_id)) {
                    if let Some(parent_name) = schema_parent.ds_object_name2_opt()? {
                        if parent_name == "Configuration" {
                            log::info!("found record id to be {}", record.ds_record_id()?);
                            return Ok(record.ds_record_id()?);
                        }
                    }
                }
            }
        }
        Err(crate::ntds::Error::MissingSchemaRecord)
    }
}
