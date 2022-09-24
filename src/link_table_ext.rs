use std::collections::{HashMap, HashSet};
use anyhow::{Result, bail};
use libesedb::{Table, Value};


/// wraps a ESEDB Table.
/// This class assumes the a NTDS link_table is being wrapped
pub(crate) struct LinkTableExt {
    forward_map: HashMap<i32, HashSet<i32>>,
    backward_map: HashMap<i32, HashSet<i32>>
}

impl LinkTableExt {
    /// create a new datatable wrapper
    pub fn from(table: Table<'_>) -> Result<Self> {
        log::info!("reading link information and creating link_table cache");

        let mut columns = HashMap::new();
        for index in 0..table.count_columns()?-1 {
            let column = table.column(index)?;
            columns.insert(column.name()?, index);
        }
        let link_dnt_id = match columns.get("link_DNT") {
            Some(v) => v,
            _ => bail!("missing link_DNT column")
        };

        let backward_dnt_id = match columns.get("backlink_DNT") {
            Some(v) => v,
            _ => bail!("missing backlink_DNT column")
        };

        let mut forward_map = HashMap::new();
        let mut backward_map = HashMap::new();
        for record in table.iter_records()
            .expect("unable to iterate this table")
            .filter_map(|r| r.ok()) {
            
                let forward_link = match record.value(*link_dnt_id)? {
                Value::I32(v) => v,
                _ => bail!("column link_DNT has an unexpected type"),
            };

            let backward_link = match record.value(*backward_dnt_id)? {
                Value::I32(v) => v,
                _ => bail!("column backlink_DNT has an unexpected type"),
            };

            forward_map.entry(forward_link).or_insert(HashSet::new()).insert(backward_link);
            backward_map.entry(backward_link).or_insert(HashSet::new()).insert(forward_link);
        }

        Ok(Self {
            forward_map,
            backward_map
        })
    }

    pub (crate) fn member_of(&self, dnt: &i32) -> Option<&HashSet<i32>> {
        self.backward_map.get(dnt)
    }

    pub (crate) fn members(&self, dnt: &i32) -> Option<&HashSet<i32>> {
        self.forward_map.get(dnt)
    }
}