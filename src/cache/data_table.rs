use crate::{cache, RecordHasAttRdn, RecordHasId};
use crate::{ntds::DataTableRecord, RecordHasParent, RecordPredicate};

use super::Iter;
pub type DataTable<'info, 'db> = cache::Table<'info, 'db>;

pub struct DataTableIter<'info, 'db>(Iter<'info, 'db>);

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

impl<'info, 'db> DataTable<'info, 'db> {
    pub fn iter_records(
        &'db self,
    ) -> impl Iterator<Item = DataTableRecord<'info, 'db>> {
        DataTableIter::from(self.iter())
    }

    pub(crate) fn children_of(
        &'db self,
        parent_id: i32,
    ) -> impl Iterator<Item = DataTableRecord<'info, 'db>> {
        let my_filter = RecordHasParent(parent_id);
        self.iter_records().filter(move |r| my_filter.matches(r))
    }

    pub fn filter<C>(
        &'db self,
        predicate: C,
    ) -> impl Iterator<Item = DataTableRecord<'info, 'db>>
    where
        C: Fn(&DataTableRecord<'info, 'db>) -> bool,
    {
        self.iter_records().filter(move |r| predicate(r))
    }

    pub fn find<C>(
        &'db self,
        predicate: C,
    ) -> Option<DataTableRecord<'info, 'db>>
    where
        C: Fn(&DataTableRecord<'info, 'db>) -> bool,
    {
        self.iter_records().find(move |r| predicate(r))
    }

    pub fn filter_p<P>(
        &'db self,
        predicate: P,
    ) -> impl Iterator<Item = DataTableRecord<'info, 'db>>
    where
        P: RecordPredicate<'info, 'db>,
    {
        self.iter_records().filter(move |r| predicate.matches(r))
    }

    pub fn find_p<P>(
        &'db self,
        predicate: P,
    ) -> Option<DataTableRecord<'info, 'db>>
    where
        P: RecordPredicate<'info, 'db>,
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
