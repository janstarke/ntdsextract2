use std::rc::Rc;

use libesedb::Table;

use super::{CColumn, CRecord};
use crate::ColumnInfoMapping;

pub struct CTable<'r> {
    records: Vec<CRecord<'r>>,
    columns: Vec<CColumn>,
}

impl<'r> CTable<'r> {
    pub fn try_from(table: Table<'r>, mapping: Rc<ColumnInfoMapping>) -> std::io::Result<Self> {
        let mut records = Vec::new();
        let mut columns = Vec::new();

        for record in table.iter_records()? {
            records.push(CRecord::try_from(record?, Rc::clone(&mapping))?);
        }
        for column in table.iter_columns()? {
            columns.push(CColumn::try_from(column?)?);
        }

        Ok(Self { records, columns })
    }

    pub fn count_columns(&self) -> i32 {
        self.columns.len().try_into().unwrap()
    }

    pub fn iter(&self) -> impl Iterator<Item = &CRecord<'r>> {
        self.records.iter()
    }

    pub fn column(&self, pos: i32) -> Option<&CColumn> {
        self.columns.get(usize::try_from(pos).unwrap())
    }
    
}
