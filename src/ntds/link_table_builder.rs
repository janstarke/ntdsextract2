use anyhow::{bail, ensure, Result};
use libesedb::Value;
use std::collections::{HashMap, HashSet};

use crate::value::FromValue;
use crate::{CDataTable, CLinkTable};

use super::LinkTable;

pub(crate) struct LinkTableBuilder<'a, 'd, 'l> {
    link_table: CLinkTable<'l>,
    data_table: &'a CDataTable<'d>,
    schema_record_id: i32,
    columns: HashMap<String, i32>,
}

impl<'a, 'd, 'l> LinkTableBuilder<'a, 'd, 'l> {
    pub fn from(
        link_table: CLinkTable<'l>,
        data_table: &'a CDataTable<'d>,
        schema_record_id: i32,
    ) -> Result<Self> {
        let columns = Self::read_column_names(&link_table)?;

        Ok(Self {
            link_table,
            data_table,
            schema_record_id,
            columns,
        })
    }

    fn read_column_names(link_table: &CLinkTable) -> Result<HashMap<String, i32>> {
        let mut columns = HashMap::new();
        for index in 0..link_table.count_columns() - 1 {
            let column = link_table.column(index).unwrap();
            columns.insert(column.name().to_owned(), index);
        }
        Ok(columns)
    }

    fn column_id(&self, name: &str) -> Result<i32> {
        match self.columns.get(name) {
            Some(id) => Ok(*id),
            None => bail!("no column by that name: '{name}'"),
        }
    }
    /*
        fn from_column<I: FromValue>(&self, record: &Record, name: &str) -> Result<I> {
            let id = self.column_id(name)?;
            I::from_value(record.value(id)?, name)
        }
    */
    pub fn build(self) -> Result<LinkTable> {
        log::info!("building link table associations");

        let (member_link_id, _member_of_link_id) = self.find_member_link_id_pair()?;
        let link_base = member_link_id / 2;
        let link_dnt_id = self.column_id("link_DNT")?;
        let backward_dnt_id = self.column_id("backlink_DNT")?;
        let link_base_id = self.column_id("link_base")?;

        let mut forward_map = HashMap::new();
        let mut backward_map = HashMap::new();

        for record in self.link_table.iter_records().filter(|r| {
            r.get_by_index(link_base_id).as_ref()
                .map_or(false, |value| match value.value() {
                    Value::U32(v) => *v == member_link_id,
                    Value::I32(v) => u32::try_from(*v).map_or(false, |v| v == link_base),
                    _ => false,
                })
        }) {
            if let Some(forward_link_value) = record.get_by_index(link_dnt_id) {
                if let Some(backward_link_value) = record.get_by_index(backward_dnt_id) {
                    let forward_link = i32::from_value(forward_link_value.value())?;
                    let backward_link = i32::from_value(backward_link_value.value())?;
                    forward_map
                        .entry(forward_link)
                        .or_insert_with(HashSet::new)
                        .insert(backward_link);
                    backward_map
                        .entry(backward_link)
                        .or_insert_with(HashSet::new)
                        .insert(forward_link);
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
            forward_map,
            backward_map,
        })
    }

    fn find_member_link_id_pair(&self) -> anyhow::Result<(u32, u32)> {
        log::info!("searching for link attributes 'Member' and 'Is-Member-Of-DL'");

        let member_link_id = self.find_link_id(&String::from("Member"))?;
        let member_of_link_id = self.find_link_id(&String::from("Is-Member-Of-DL"))?;

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
        Ok(self
            .data_table
            .children_of(self.schema_record_id)
            .find(|r| &r.ds_object_name2().expect("missing object_name2") == attribute_name)
            .expect(&format!("found no record by that name: '{attribute_name}'"))
            .ds_link_id()?)
    }
}
