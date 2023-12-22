use std::{ops::Index, collections::HashMap};

use crate::{column_information::ColumnInformation, ntds::NtdsAttributeId};
use anyhow::Result;
use libesedb::Table;

pub struct ColumnInfoMapping {
    mapping: HashMap<NtdsAttributeId, ColumnInformation>,
}

impl Index<NtdsAttributeId> for ColumnInfoMapping {
    type Output = ColumnInformation;

    fn index(&self, index: NtdsAttributeId) -> &Self::Output {
        self.mapping.index(&index)
    }
}

impl TryFrom<&Table<'_>> for ColumnInfoMapping {
    type Error = anyhow::Error;
    fn try_from(data_table: &Table) -> Result<Self, Self::Error> {
        let mut mapping = HashMap::new();
        for index in 0..data_table.count_columns()? {
            let column = data_table.column(index)?;
            if let Ok(column_id) = NtdsAttributeId::try_from(&column.name()?[..]) {
                let col_info = ColumnInformation::new(
                    index,
                    // column_res.name()?,
                    // column_res.variant()?
                );
                mapping.insert(column_id, col_info);
            }
            //log::info!("found column with name {name}", name=column_res.name());
        }

        Ok(Self { mapping })
    }
}