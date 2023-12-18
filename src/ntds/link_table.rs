use anyhow::Result;
use std::collections::{HashMap, HashSet};

use crate::cache;
use crate::cache::RecordPointer;
use crate::ntds::link_table_builder::LinkTableBuilder;

use super::DataTable;

/// wraps a ESEDB Table.
/// This class assumes the a NTDS link_table is being wrapped
pub struct LinkTable {
    pub(crate) _forward_map: HashMap<RecordPointer, HashSet<RecordPointer>>,
    pub(crate) backward_map: HashMap<RecordPointer, HashSet<RecordPointer>>,
}

impl LinkTable {
    /// create a new datatable wrapper
    pub fn new<'info, 'db>(
        link_table: cache::Table<'info, 'db, cache::LinkTable>,
        data_table: &cache::Table<'info, 'db, cache::DataTable>,
        schema_record_id: RecordPointer,
    ) -> Result<Self> {
        log::info!("reading link information and creating link_table cache");

        let builder = LinkTableBuilder::from(link_table, data_table, schema_record_id)?;
        builder.build()
    }

    pub(crate) fn member_of(&self, dnt: &RecordPointer) -> Option<&HashSet<RecordPointer>> {
        self.backward_map.get(dnt)
    }

    pub fn member_names_of(
        &self,
        object_id: RecordPointer,
        data_table: &DataTable<'_, '_>,
    ) -> Vec<String> {
        let member_of = if let Some(children) = self.member_of(&object_id) {
            children
                .iter()
                .filter_map(|child_id| data_table.data_table().find_by_id(*child_id))
                .map(|record| {
                    record
                        .att_object_name2()
                        .expect("error while reading object name")
                })
                .collect()
        } else {
            vec![]
        };
        member_of
    }

    /*
    pub(crate) fn members(&self, dnt: &i32) -> Option<&HashSet<i32>> {
        self.forward_map.get(dnt)
    }
     */
}
