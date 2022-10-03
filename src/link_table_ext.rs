use anyhow::{bail, ensure, Result};
use libesedb::{Table, Value};
use std::collections::{HashMap, HashSet};

use crate::{column_info_mapping::ColumnInfoMapping, data_table_ext::DataTableExt};

/// wraps a ESEDB Table.
/// This class assumes the a NTDS link_table is being wrapped
pub(crate) struct LinkTableExt {
    forward_map: HashMap<i32, HashSet<i32>>,
    backward_map: HashMap<i32, HashSet<i32>>,
}

struct LinkTableBuilder<'a, 'b, 'c> {
    link_table: Table<'b>,
    data_table: &'a Table<'b>,
    mapping: &'c ColumnInfoMapping,
    schema_record_id: i32,
}

impl<'a, 'b, 'c> LinkTableBuilder<'a, 'b, 'c> {
    pub fn from(
        link_table: Table<'b>,
        data_table: &'a Table<'b>,
        mapping: &'c ColumnInfoMapping,
        schema_record_id: i32,
    ) -> Self {
        Self {
            link_table,
            data_table,
            mapping,
            schema_record_id,
        }
    }

    pub fn build(self) -> Result<LinkTableExt> {
        let (member_link_id, member_of_link_id) =
            self.find_member_link_id_pair()?;

        let mut columns = HashMap::new();
        for index in 0..self.link_table.count_columns()? - 1 {
            let column = self.link_table.column(index)?;
            columns.insert(column.name()?, index);
        }
        let link_dnt_id = match columns.get("link_DNT") {
            Some(v) => v,
            _ => bail!("missing link_DNT column"),
        };

        let backward_dnt_id = match columns.get("backlink_DNT") {
            Some(v) => v,
            _ => bail!("missing backlink_DNT column"),
        };

        let mut forward_map = HashMap::new();
        let mut backward_map = HashMap::new();
        for record in self.link_table
            .iter_records()
            .expect("unable to iterate this table")
            .filter_map(|r| r.ok())
        {
            let forward_link = match record.value(*link_dnt_id)? {
                Value::I32(v) => v,
                _ => bail!("column link_DNT has an unexpected type"),
            };

            let backward_link = match record.value(*backward_dnt_id)? {
                Value::I32(v) => v,
                _ => bail!("column backlink_DNT has an unexpected type"),
            };

            forward_map
                .entry(forward_link)
                .or_insert_with(HashSet::new)
                .insert(backward_link);
            backward_map
                .entry(backward_link)
                .or_insert_with(HashSet::new)
                .insert(forward_link);
        }

        for (key, value) in forward_map.iter() {
            log::info!("found link {key} --> {value:?}");
        }

        Ok(LinkTableExt {
            forward_map,
            backward_map,
        })
    }

    fn find_member_link_id_pair(&self) -> Result<(u32, u32)> {
        let member_link_id = self.find_link_id(&String::from("Member"))?;
        let member_of_link_id = self.find_link_id(&String::from("Is-Member-Of-DL"))?;

        ensure!(
            member_link_id + 1 == member_of_link_id,
            "invalid LinkID values: {} and {}",
            member_link_id,
            member_of_link_id
        );

        Ok((member_link_id, member_of_link_id))
    }

    fn find_link_id(&self, attribute_name: &String) -> Result<u32> {
        match DataTableExt::find_children_of(self.data_table, self.mapping, self.schema_record_id)
            .find(|r| r.ds_object_name2_equals(self.mapping, attribute_name))
        {
            Some(record) => match record.ds_link_id(self.mapping)? {
                Some(id) => Ok(id),
                None => bail!("record '{}' (name is '{attribute_name}') has no LinkId attribute", record.ds_record_id(self.mapping)?.unwrap()),
            },
            None => bail!("found no record by that name: '{attribute_name}'"),
        }
    }
}

impl LinkTableExt {
    /// create a new datatable wrapper
    pub fn from(
        link_table: Table<'_>,
        data_table: &Table<'_>,
        mapping: &ColumnInfoMapping,
        schema_record_id: i32,
    ) -> Result<Self> {
        log::info!("reading link information and creating link_table cache");

        let builder = LinkTableBuilder::from(link_table, data_table, mapping, schema_record_id);
        builder.build()
    }

    pub(crate) fn member_of(&self, dnt: &i32) -> Option<&HashSet<i32>> {
        self.backward_map.get(dnt)
    }

    pub(crate) fn members(&self, dnt: &i32) -> Option<&HashSet<i32>> {
        self.forward_map.get(dnt)
    }
}
