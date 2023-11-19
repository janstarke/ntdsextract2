use anyhow::Result;
use std::{path::PathBuf, rc::Rc};

use libesedb::EseDb;

use crate::{
    column_info_mapping::ColumnInfoMapping, link_table_ext::LinkTableExt,
    object_tree_entry::ObjectTreeEntry, CTable, DataTableExt,
};

pub struct CDatabase<'r> {
    data_table: Rc<DataTableExt<'r>>,
    link_table: Rc<LinkTableExt>,
}

impl<'a> CDatabase<'a> {
    pub fn from_path(value: &PathBuf) -> Result<Rc<Self>> {
        let esedb = EseDb::open(&value)?;
        log::info!("Db load finished");
        Self::from_db(esedb)
    }

    pub fn from_db(esedb: EseDb) -> Result<Rc<Self>> {
        let raw_data_table = CTable::try_from(esedb.table_by_name("datatable")?)?;
        log::info!("cached data_table");

        let raw_link_table = CTable::try_from(esedb.table_by_name("link_table")?)?;
        log::info!("cached link_table");

        log::info!("reading schema information and creating record cache");
        let mapping = ColumnInfoMapping::from(&raw_data_table)?;
        let object_tree = ObjectTreeEntry::from(&raw_data_table, &mapping)?;
        let schema_record_id = raw_data_table.get_schema_record_id(&mapping)?;

        log::debug!("found the schema record id is '{}'", schema_record_id);

        let link_table = Rc::new(LinkTableExt::new(
            raw_link_table,
            &raw_data_table,
            &mapping,
            schema_record_id,
        )?);

        let data_table = Rc::new(DataTableExt::new(raw_data_table, mapping, object_tree, schema_record_id)?);

        let me = Rc::new(Self {
            data_table,
            link_table,
        });

        //me.data_table.set_database(me.downgrade());

        Ok(me)
    }
    pub fn data_table(&self) -> &DataTableExt<'a> {
        &self.data_table
    }
    
    pub fn link_table(&self) -> &LinkTableExt {
        &self.link_table
    }
}
