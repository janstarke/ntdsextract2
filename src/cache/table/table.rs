use std::rc::Rc;

use crate::{cache, ColumnInfoMapping};
use ouroboros::self_referencing;

use super::{Iter, EsedbTable};

#[self_referencing]
pub struct Table<'table, 'record: 'table>
{
    table_id: &'static str,

    table: libesedb::Table<'table>,

    columns: Vec<cache::Column>,

    #[borrows(table)]
    records: Vec<cache::Record<'record>>,
}

impl<'table, 'record:'table> EsedbTable<'table, 'record> for Table<'table, 'record>
{
    fn iter<'s> (&'s self) -> Iter<'s, 'record> {
        self.borrow_records().iter().into()
    }
}

impl<'table, 'record:'table> Table<'table, 'record>
{
    pub fn try_from(
        table: libesedb::Table<'table>,
        table_id: &'static str,
        mapping: Rc<ColumnInfoMapping>,
    ) -> std::io::Result<Self> {
        let mut columns = Vec::new();

        for column in table.iter_columns()? {
            columns.push(cache::Column::try_from(column?)?);
        }
        let x = TableBuilder {
            table,
            table_id,
            records_builder: |table| {
                let mut records = Vec::new();
                let mut record_id = 0;
                for record in table.iter_records().unwrap() {
                    records.push(
                        cache::Record::try_from(
                            record.unwrap(),
                            table_id,
                            record_id,
                            Rc::clone(&mapping),
                        )
                        .unwrap(),
                    );
                    record_id += 1;
                }
                records
            },
            columns,
        }
        .build();

        Ok(x)
    }

    pub fn count_columns(&self) -> i32 {
        self.borrow_columns().len().try_into().unwrap()
    }

    pub fn column(&self, pos: i32) -> Option<&cache::Column> {
        self.borrow_columns().get(usize::try_from(pos).unwrap())
    }
}
