use anyhow::Result;
use std::collections::{HashMap, HashSet};

use crate::ntds::link_table_builder::LinkTableBuilder;
use crate::{CDataTable, CLinkTable, CRecord, EsedbTable};

/// wraps a ESEDB Table.
/// This class assumes the a NTDS link_table is being wrapped
pub struct LinkTable {
    pub(crate) forward_map: HashMap<i32, HashSet<i32>>,
    pub(crate) backward_map: HashMap<i32, HashSet<i32>>,
}

impl LinkTable {
    /// create a new datatable wrapper
    pub fn new<LT, DT>(link_table: LT, data_table: &DT, schema_record_id: i32) -> Result<Self>
    where
        for<'lt, 'lr> LT: EsedbTable<'lt, CRecord<'lr>>,
        for<'dt, 'dr> DT: EsedbTable<'dt, CRecord<'dr>>,
    {
        log::info!("reading link information and creating link_table cache");

        let builder = LinkTableBuilder::from(link_table, data_table, schema_record_id)?;
        builder.build()
    }

    pub(crate) fn member_of(&self, dnt: &i32) -> Option<&HashSet<i32>> {
        self.backward_map.get(dnt)
    }

    pub(crate) fn members(&self, dnt: &i32) -> Option<&HashSet<i32>> {
        self.forward_map.get(dnt)
    }
}
