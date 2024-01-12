use std::{collections::HashMap, ops::Index};

use crate::{column_information::ColumnInformation, ntds::NtdsAttributeId};
use anyhow::Result;
use libesedb::Table;

pub struct ColumnInfoMapping {
    mapping: HashMap<NtdsAttributeId, ColumnInformation>,
    str_mapping: HashMap<String, ColumnInformation>,
}

impl Index<NtdsAttributeId> for ColumnInfoMapping {
    type Output = ColumnInformation;

    fn index(&self, index: NtdsAttributeId) -> &Self::Output {
        self.mapping.index(&index)
    }
}

impl ColumnInfoMapping {
    pub fn info_by_name(&self, index: &str) -> Option<&ColumnInformation> {
        self.str_mapping.get(index)
    }
}

impl TryFrom<&Table<'_>> for ColumnInfoMapping {
    type Error = anyhow::Error;
    fn try_from(data_table: &Table) -> Result<Self, Self::Error> {
        let mut mapping = HashMap::new();
        let mut str_mapping = HashMap::new();
        for index in 0..data_table.count_columns()? {
            let column = data_table.column(index)?;
            let col_info = ColumnInformation::new(index);
            if let Ok(column_id) = NtdsAttributeId::try_from(&column.name()?[..]) {
                mapping.insert(column_id, col_info);
            }

            str_mapping.insert(column.name()?.to_string(), col_info);
        }

        Ok(Self {
            mapping,
            str_mapping,
        })
    }
}
