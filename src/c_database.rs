use anyhow::Result;
use std::{path::PathBuf, rc::Rc};

use libesedb::EseDb;

use crate::{
    ntds::{self, DataTable, LinkTable},
    object_tree_entry::ObjectTreeEntry,
    CTable, ColumnInfoMapping,
};

pub struct CDatabase<'r> {
    data_table: Rc<DataTable<'r>>,
    link_table: Rc<LinkTable>,
    esedb: EseDb
}

impl<'a> CDatabase<'a> {
    pub fn from_path(value: &PathBuf) -> Result<Rc<Self>> {
        let esedb = EseDb::open(&value)?;
        log::info!("Db load finished");
        Self::from_db(esedb)
    }

    pub fn from_db(esedb: EseDb) -> Result<Rc<Self>> {
        let esedb_data_table = esedb.table_by_name("datatable")?;
        let mapping = Rc::new(ColumnInfoMapping::try_from(&esedb_data_table)?);

        let raw_data_table = CTable::try_from(esedb_data_table, Rc::clone(&mapping))?;
        log::info!("cached data_table");

        let raw_link_table = CTable::try_from(esedb.table_by_name("link_table")?, Rc::clone(&mapping))?;
        log::info!("cached link_table");

        log::info!("reading schema information and creating record cache");
        let object_tree = ObjectTreeEntry::from(&raw_data_table)?;
        let schema_record_id = ntds::DataTable::get_schema_record_id(&raw_data_table)?;

        log::debug!("found the schema record id is '{}'", schema_record_id);

        let link_table = Rc::new(LinkTable::new(
            raw_link_table,
            &raw_data_table,
            schema_record_id,
        )?);

        let data_table = Rc::new(DataTable::new(
            raw_data_table,
            object_tree,
            schema_record_id,
        )?);

        let me = Rc::new(Self {
            data_table,
            link_table,
            esedb
        });

        //me.data_table.set_database(me.downgrade());

        Ok(me)
    }
    pub fn data_table(&self) -> &DataTable<'a> {
        &self.data_table
    }

    pub fn link_table(&self) -> &LinkTable {
        &self.link_table
    }
}
