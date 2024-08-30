use anyhow::Result;
use std::collections::{HashMap, HashSet};

use crate::cache::RecordPointer;
use crate::cache::{self, RecordId};
use crate::ntds::link_table_builder::LinkTableBuilder;
use crate::win32_types::Rdn;
use crate::{Membership, MembershipSet, SerializationType};

use super::DataTable;

/// wraps a ESEDB Table.
/// This class assumes the a NTDS link_table is being wrapped
pub struct LinkTable {
    pub(crate) _forward_map: HashMap<RecordId, HashSet<RecordPointer>>,
    pub(crate) backward_map: HashMap<RecordId, HashSet<RecordPointer>>,
}

impl LinkTable {
    /// create a new datatable wrapper
    pub fn new<'info, 'db>(
        link_table: cache::LinkTable<'info, 'db>,
        data_table: &cache::DataTable<'info, 'db>,
        schema_record_id: RecordPointer,
    ) -> Result<Self> {
        log::info!("reading link information and creating link_table cache");

        let builder = LinkTableBuilder::from(link_table, data_table, schema_record_id)?;
        builder.build(data_table.metadata())
    }

    pub(crate) fn member_of(&self, dnt: &RecordId) -> Option<&HashSet<RecordPointer>> {
        self.backward_map.get(dnt)
    }

    pub fn member_names_of(&self, object_id: RecordId, data_table: &DataTable<'_, '_>) -> Vec<Rdn> {
        let member_of = if let Some(children) = self.member_of(&object_id) {
            children
                .iter()
                .map(|child_id| &data_table.data_table().metadata()[child_id])
                .map(|record| record.rdn().clone())
                .collect()
        } else {
            vec![]
        };
        member_of
    }

    pub fn member_refs_of<T: SerializationType>(
        &self,
        object_id: RecordId,
        data_table: &DataTable<'_, '_>,
    ) -> MembershipSet<T> {
        let member_of = if let Some(children) = self.member_of(&object_id) {
            children
                .iter()
                .map(|child_id| &data_table.data_table().metadata()[child_id])
                .map(|record| {
                    (
                        *record.record_ptr(),
                        record.rdn().clone(),
                        record.sid().clone(),
                        None,
                    )
                })
                .collect()
        } else {
            vec![]
        };
        MembershipSet::<T>::from(member_of.into_iter().map(Membership::from))
    }
}
