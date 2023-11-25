use std::rc::Rc;

use libesedb::Table;

use super::{CColumn, CRecord};
use crate::{ColumnInfoMapping, EsedbRecord};
use ouroboros::self_referencing;

pub trait RecordIterator<'r>: Iterator<Item = &'r CRecord<'r>> {}

pub trait EsedbTable<'table, R>
where
    for<'record> R: EsedbRecord<'record>,
{
    fn iter<I>(&self) -> I
    where
        for<'r> I: RecordIterator<'r>;
}

#[self_referencing]
pub struct CTable<'table, R>
where
    for<'record> R: EsedbRecord<'record>,
{
    table: Table<'table>,

    columns: Vec<CColumn>,

    #[borrows(table)]
    records: Vec<R>,
}

impl<'table, R> EsedbTable<'table, R> for CTable<'table, R>
where
    for<'record> R: EsedbRecord<'record>,
{
    fn iter<I>(&self) -> I
    where
        for<'r> I: RecordIterator<'r>,
    {
        self.borrow_records().iter()
    }
}

impl<'table, R> CTable<'table, R>
where
    for<'record> R: EsedbRecord<'record>,
{
    pub fn try_from(table: Table<'table>, mapping: Rc<ColumnInfoMapping>) -> std::io::Result<Self> {
        let mut columns = Vec::new();

        for column in table.iter_columns()? {
            columns.push(CColumn::try_from(column?)?);
        }

        Ok(CTableBuilder {
            table,
            records_builder: |table| {
                let mut records = Vec::new();
                for record in table.iter_records().unwrap() {
                    records.push(CRecord::try_from(record.unwrap(), Rc::clone(&mapping)).unwrap());
                }
                records
            },
            columns,
        }
        .build())
    }

    pub fn count_columns(&self) -> i32 {
        self.borrow_columns().len().try_into().unwrap()
    }

    pub fn column(&self, pos: i32) -> Option<&CColumn> {
        self.borrow_columns().get(usize::try_from(pos).unwrap())
    }
}
