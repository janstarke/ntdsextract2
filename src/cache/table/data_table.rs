use getset::Getters;

use std::rc::Rc;

use crate::{
    cache::{self, ColumnsOfTable, MetaDataCache},
    esedb_mitigation::libesedb_count,
    ntds::DataTableRecord,
    object_tree_entry::ObjectTreeEntry,
    EsedbInfo,
};

use super::RecordPointer;

#[derive(Getters)]
#[getset(get = "pub")]
pub struct DataTable<'info, 'db>
where
    'info: 'db,
{
    table_id: &'static str,

    table: &'info libesedb::Table<'db>,

    esedbinfo: &'info EsedbInfo<'db>,
    metadata: MetaDataCache,

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
            table,
            table_id,
            esedbinfo,
            metadata,
            number_of_records: libesedb_count(|| table.count_records())?,
            columns: Rc::new(ColumnsOfTable::try_from(table)?),
        })
    }
}

impl<'info, 'db> DataTable<'info, 'db> {
    pub fn iter(&'db self) -> impl Iterator<Item = DataTableRecord<'info, 'db>> {
        self.metadata
            .iter()
            .map(|e| self.data_table_record_from(*e.record_ptr()))
            .filter_map(Result::ok)
    }

    pub fn data_table_record_from(
        &self,
        ptr: RecordPointer,
    ) -> std::io::Result<DataTableRecord<'info, 'db>> {
        Ok(DataTableRecord::new(
            cache::Record::try_from(
                self.table.record(ptr.esedb_row().inner())?,
                self.table_id,
                *ptr.esedb_row(),
                self.esedbinfo,
                Rc::clone(&self.columns),
            )?,
            ptr,
        ))
    }
    pub fn path_to_str(&self, path: &[Rc<ObjectTreeEntry>]) -> String {
        let v: Vec<_> = path.iter().map(|e| e.name().to_string()).collect();
        v.join(",")
    }
}
