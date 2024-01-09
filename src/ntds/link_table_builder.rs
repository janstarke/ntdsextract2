use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, ensure, Result};

use crate::cache::{self, MetaDataCache, RecordId, RecordPointer, Value, WithValue};
use crate::value::FromValue;

use super::LinkTable;

pub(crate) struct LinkTableBuilder<'info, 'db> {
    link_table: cache::LinkTable<'info, 'db>,
    data_table: &'db cache::DataTable<'info, 'db>,
    schema_record_id: RecordPointer,
}

impl<'info, 'db> LinkTableBuilder<'info, 'db> {
    pub fn from(
        link_table: cache::LinkTable<'info, 'db>,
        data_table: &'db cache::DataTable<'info, 'db>,
        schema_record_id: RecordPointer,
    ) -> Result<Self> {
        Ok(Self {
            link_table,
            data_table,
            schema_record_id,
        })
    }

    pub fn build(self, metadata: &MetaDataCache) -> Result<LinkTable> {
        log::info!("building link table associations");

        let (member_link_id, _member_of_link_id) = self.find_member_link_id_pair()?;
        let link_base = member_link_id / 2;
        let link_dnt_id = self.link_table.link_dnt_id();
        let backlink_dnt_id = self.link_table.backlink_dnt_id();
        let link_base_id = self.link_table.link_base_id();

        let mut forward_map = HashMap::new();
        let mut backward_map = HashMap::new();

        for record in self.link_table.iter().filter(|r| {
            r.with_value(*link_base_id, |value| match value {
                Some(Value::U32(v)) => Ok(*v == member_link_id),
                Some(Value::I32(v)) => Ok(u32::try_from(*v).map_or(false, |v| v == link_base)),
                _ => Ok(false),
            })
            .unwrap_or(false)
        }) {
            if let Ok(Some(forward_link)) = record.with_value(*link_dnt_id, |v| {
                RecordId::from_value(v.unwrap())
                    .map_err(|e| anyhow!(e))
                    .map(|id| {
                        metadata.ptr_from_id(&id).or_else(|| {
                            log::warn!("I expected to find an entry for forward link {id}; but there was none. I'll ignore that entry.");
                            None
                        })
                    })
            }) {
                if let Ok(Some(backward_link)) = record.with_value(*backlink_dnt_id, |v| {
                    RecordId::from_value(v.unwrap())
                        .map_err(|e| anyhow!(e))
                        .map(|id| {
                            metadata.ptr_from_id(&id).or_else(|| {
                                log::warn!(
                                    "I expected to find an entry for backward link {id}; but there was none. I'll ignore that entry."
                                );
                                None
                            })
                        })
                }) {
                    forward_map
                        .entry(*forward_link.ds_record_id())
                        .or_insert_with(HashSet::new)
                        .insert(backward_link.clone());
                    backward_map
                        .entry(*backward_link.ds_record_id())
                        .or_insert_with(HashSet::new)
                        .insert(forward_link.clone());
                }
            }
        }

        for (key, value) in forward_map.iter() {
            log::info!("found link {key} --> {value:?}");
        }

        for (key, value) in backward_map.iter() {
            log::info!("found backlink {key} --> {value:?}");
        }

        log::debug!(
            "found {} forward links and {} backward links",
            forward_map.len(),
            backward_map.len()
        );

        Ok(LinkTable {
            _forward_map: forward_map,
            backward_map,
        })
    }

    fn find_member_link_id_pair(&self) -> anyhow::Result<(u32, u32)> {
        log::info!("searching for link attributes 'Member' and 'Is-Member-Of-DL'");

        let member_link_id = self.find_link_id(&String::from("Member"))?;
        log::info!("'Member' has Link-ID '{member_link_id}'");

        let member_of_link_id = self.find_link_id(&String::from("Is-Member-Of-DL"))?;
        log::info!("'Is-Member-Of-DL' has Link-ID '{member_of_link_id}'");

        ensure!(
            member_link_id & 1 == 0,
            "the forward LinkID must be a even number"
        );

        ensure!(
            member_link_id + 1 == member_of_link_id,
            "invalid LinkID values: {} and {}",
            member_link_id,
            member_of_link_id
        );

        Ok((member_link_id, member_of_link_id))
    }

    fn find_link_id(&self, attribute_name: &String) -> Result<u32> {
        self.data_table
            .children_of(self.schema_record_id)
            .find(|r| r.att_object_name2().expect("missing object_name2").name() == attribute_name)
            .unwrap_or_else(|| panic!("found no record by that name: '{attribute_name}'"))
            .att_link_id()
    }
}
