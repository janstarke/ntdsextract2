use anyhow::anyhow;
use getset::Getters;

use std::rc::Rc;

use crate::{
    cache::{self, EsedbRowId, MetaDataCache, RecordId, ColumnsOfTable},
    ntds::DataTableRecord,
    object_tree_entry::ObjectTreeEntry,
    EsedbInfo, RecordHasParent, RecordPredicate,
};

use super::{RecordPointer, SpecialRecords};

#[derive(Getters)]
pub struct DataTable<'info, 'db>
where
    'info: 'db,
{
    _table_id: &'static str,
    _table: &'info libesedb::Table<'db>,
    esedbinfo: &'info EsedbInfo<'db>,

    #[getset(get="pub")]
    metadata: MetaDataCache,

    #[getset(get="pub")]
    number_of_records: i32,

    // this is needed for `::all_atributes`
    columns: Rc<ColumnsOfTable>,
}

impl<'info, 'db> DataTable<'info, 'db>
where
    'info: 'db,
{
    pub fn new(
        table: &'info libesedb::Table<'db>,
        table_id: &'static str,
        esedbinfo: &'info EsedbInfo<'db>,
        metadata: MetaDataCache,
    ) -> std::io::Result<Self> {
        Ok(Self {
            _table: table,
            _table_id: table_id,
            esedbinfo,
            metadata,
            number_of_records: table.count_records()?,
            columns: Rc::new(ColumnsOfTable::try_from(table)?)
        })
    }
}

impl<'info, 'db> DataTable<'info, 'db> {
    pub fn iter(&'db self) -> impl Iterator<Item = DataTableRecord<'info, 'db>> {
        (0..self.number_of_records).map(|row| self.data_table_record_from(row.into()).unwrap())
    }

    fn data_table_record_from(
        &self,
        row: EsedbRowId,
    ) -> std::io::Result<DataTableRecord<'info, 'db>> {
        Ok(DataTableRecord::new(
            cache::Record::try_from(
                self._table.record(row.inner())?,
                self._table_id,
                row,
                self.esedbinfo,
                Rc::clone(&self.columns)
            )?,
            row,
        ))
    }

    pub(crate) fn children_of(
        &'db self,
        parent_id: RecordPointer,
    ) -> impl Iterator<Item = DataTableRecord<'info, 'db>> {
        let my_filter = RecordHasParent(*parent_id.ds_record_id());
        self.iter().filter(move |r| my_filter.matches(r))
    }

    pub fn filter<C>(&'db self, predicate: C) -> impl Iterator<Item = DataTableRecord<'info, 'db>>
    where
        C: Fn(&DataTableRecord<'info, 'db>) -> bool,
    {
        self.iter().filter(move |r| predicate(r))
    }

    pub fn find<C>(&'db self, predicate: C) -> Option<DataTableRecord<'info, 'db>>
    where
        C: Fn(&DataTableRecord<'info, 'db>) -> bool,
    {
        self.iter().find(move |r| predicate(r))
    }

    pub fn filter_p<P>(&'db self, predicate: P) -> impl Iterator<Item = DataTableRecord<'info, 'db>>
    where
        P: RecordPredicate<'info, 'db>,
    {
        self.iter().filter(move |r| predicate.matches(r))
    }

    pub fn find_p<P>(&'db self, predicate: P) -> Option<DataTableRecord<'info, 'db>>
    where
        P: RecordPredicate<'info, 'db>,
    {
        self.iter().find(move |r| predicate.matches(r))
    }

    pub fn get_special_records(&self, root: Rc<ObjectTreeEntry>) -> anyhow::Result<SpecialRecords> {
        log::info!("obtaining special record ids");

        // search downward until we find a `Configuration` entry
        let configuration_path = ObjectTreeEntry::find_first_in_tree(&root, "Configuration")
            .ok_or(anyhow!("db has no `Configuration` entry"))?;

        let schema_subpath = configuration_path[0]
            .find_child_by_name("Schema")
            .ok_or(anyhow!("db has no `Schema` entry"))?;

        let deleted_objects_subpath = configuration_path[0]
            .find_child_by_name("Deleted Objects")
            .ok_or(anyhow!("db has no `Deleted Objects` entry"))?;

        Ok(SpecialRecords::new(schema_subpath, deleted_objects_subpath))
    }

    pub fn path_to_str(&self, path: &Vec<Rc<ObjectTreeEntry>>) -> String {
        let v: Vec<_> = path.iter().map(|e| e.name().to_owned()).collect();
        v.join(",")
    }
}

pub trait FindRecord<'info, 'db, T>
where
    'info: 'db
{
    fn find_record(&'db self, id: &T) -> std::io::Result<DataTableRecord<'info, 'db>>;
}

impl<'info, 'db> FindRecord<'info, 'db, EsedbRowId> for DataTable<'info, 'db>
where
    'info: 'db
{
    fn find_record(&'db self, id: &EsedbRowId) -> std::io::Result<DataTableRecord<'info, 'db>> {
        self.data_table_record_from(*id)
    }
}

impl<'info, 'db> FindRecord<'info, 'db, RecordId> for DataTable<'info, 'db>
where
    'info: 'db,
{
    fn find_record(&'db self, id: &RecordId) -> std::io::Result<DataTableRecord<'info, 'db>> {
        self.find_record(self.metadata.ptr_from_id(id).esedb_row())
    }
}

impl<'info, 'db> FindRecord<'info, 'db, RecordPointer> for DataTable<'info, 'db>
where
    'info: 'db,
{
    fn find_record(&'db self, id: &RecordPointer) -> std::io::Result<DataTableRecord<'info, 'db>> {
        self.data_table_record_from(*id.esedb_row())
    }
}