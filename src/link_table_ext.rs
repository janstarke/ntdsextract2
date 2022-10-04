use anyhow::{bail, ensure, Result};
use libesedb::{Table, Value, Record};
use std::collections::{HashMap, HashSet};

use crate::{column_info_mapping::ColumnInfoMapping, data_table_ext::DataTableExt};
use crate::esedb_utils::FromValue;

struct LinkTableBuilder<'a, 'b, 'c> {
    link_table: Table<'b>,
    data_table: &'a Table<'b>,
    mapping: &'c ColumnInfoMapping,
    schema_record_id: i32,
    columns: HashMap<String, i32>,
}

impl<'a, 'b, 'c> LinkTableBuilder<'a, 'b, 'c> {
    pub fn from(
        link_table: Table<'b>,
        data_table: &'a Table<'b>,
        mapping: &'c ColumnInfoMapping,
        schema_record_id: i32,
    ) -> Result<Self> {
        let columns = Self::read_column_names(&link_table)?;

        Ok(Self {
            link_table,
            data_table,
            mapping,
            schema_record_id,
            columns,
        })
    }

    fn read_column_names(link_table: &Table<'_>) -> Result<HashMap<String, i32>> {
        let mut columns = HashMap::new();
        for index in 0..link_table.count_columns()? - 1 {
            let column = link_table.column(index)?;
            columns.insert(column.name()?, index);
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
    pub fn build(self) -> Result<LinkTableExt> {
        log::info!("building link table associations");

        let (member_link_id, member_of_link_id) = self.find_member_link_id_pair()?;
        let link_base = member_link_id / 2;
        let link_dnt_id = self.column_id("link_DNT")?;
        let backward_dnt_id = self.column_id("backlink_DNT")?;
        let link_base_id = self.column_id("link_base")?;

        let mut forward_map = HashMap::new();
        let mut backward_map = HashMap::new();

        for record in self
            .link_table
            .iter_records()
            .expect("unable to iterate this table")
            .filter_map(|r| r.ok())
            .filter(|r| match 
              r.value(link_base_id){
                Ok(Value::U32(v)) => v == member_link_id,
                Ok(Value::I32(v)) => {
                  let v: Result<u32, std::num::TryFromIntError> = v.try_into();
                  match v {
                    Ok(v) => v == link_base,
                    _ => false
                  }
                }
                _ => false,
            })
        {
            let forward_link = i32::from_value(record.value(link_dnt_id)?, "link_DNT")?;
            let backward_link = i32::from_value(record.value(backward_dnt_id)?, "backlink_DNT")?;

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

        for (key, value) in backward_map.iter() {
            log::info!("found backlink {key} --> {value:?}");
        }

        log::debug!("found {} forward links and {} backward links", forward_map.len(), backward_map.len());

        Ok(LinkTableExt {
            forward_map,
            backward_map,
        })
    }

    fn find_member_link_id_pair(&self) -> Result<(u32, u32)> {
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
        match DataTableExt::find_children_of(self.data_table, self.mapping, self.schema_record_id)
            .find(|r| r.ds_object_name2_equals(self.mapping, attribute_name))
        {
            Some(record) => match record.ds_link_id(self.mapping)? {
                Some(id) => Ok(id),
                None => bail!(
                    "record '{}' (name is '{attribute_name}') has no LinkId attribute",
                    record.ds_record_id(self.mapping)?.unwrap()
                ),
            },
            None => bail!("found no record by that name: '{attribute_name}'"),
        }
    }
}

/// wraps a ESEDB Table.
/// This class assumes the a NTDS link_table is being wrapped
pub(crate) struct LinkTableExt {
    forward_map: HashMap<i32, HashSet<i32>>,
    backward_map: HashMap<i32, HashSet<i32>>,
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

        let builder = LinkTableBuilder::from(link_table, data_table, mapping, schema_record_id)?;
        builder.build()
    }

    pub(crate) fn member_of(&self, dnt: &i32) -> Option<&HashSet<i32>> {
        self.backward_map.get(dnt)
    }

    pub(crate) fn members(&self, dnt: &i32) -> Option<&HashSet<i32>> {
        self.forward_map.get(dnt)
    }
}
