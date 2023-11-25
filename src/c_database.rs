use anyhow::Result;
use std::rc::Rc;

use libesedb::EseDb;

use crate::{
    ntds::{self, DataTable, LinkTable},
    object_tree_entry::ObjectTreeEntry,
    CRecord, CTable, ColumnInfoMapping, EsedbTable,
};
use ouroboros::self_referencing;

#[self_referencing]
pub struct CDatabase<DT>
where
    for<'dtable, 'record> DT: EsedbTable<'dtable, CRecord<'record>> + 'dtable,
{
    raw_data_table: DT,

    #[borrows(raw_data_table)]
    #[covariant]
    data_table: DataTable<'this, DT>,

    link_table: LinkTable,
}

impl<DT> TryFrom<&EseDb> for CDatabase<DT>
where
    for<'dtable, 'record> DT: EsedbTable<'dtable, CRecord<'record>> + 'dtable,
{
    type Error = anyhow::Error;

    fn try_from(esedb: &EseDb) -> Result<Self, Self::Error> {
        let esedb_data_table = esedb.table_by_name("datatable")?;
        let mapping = Rc::new(ColumnInfoMapping::try_from(&esedb_data_table)?);

        let raw_data_table = CTable::try_from(esedb_data_table, Rc::clone(&mapping))?;
        log::info!("cached data_table");

        let raw_link_table =
            CTable::try_from(esedb.table_by_name("link_table")?, Rc::clone(&mapping))?;
        log::info!("cached link_table");

        log::info!("reading schema information and creating record cache");
        let object_tree = ObjectTreeEntry::from(&raw_data_table)?;
        let schema_record_id = ntds::DataTable::get_schema_record_id(&raw_data_table)?;

        log::debug!("found the schema record id is '{}'", schema_record_id);

        let link_table = LinkTable::new(raw_link_table, &raw_data_table, schema_record_id)?;
        //let data_table = DataTable::new(raw_data_table, object_tree, schema_record_id)?;
        Ok(CDatabaseBuilder {
            raw_data_table,
            data_table_builder: |raw_data_table| {
                DataTable::new(raw_data_table, object_tree, schema_record_id).unwrap()
            },
            link_table,
        }
        .build())
    }
}
