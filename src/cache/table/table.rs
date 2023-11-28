use std::rc::Rc;

use crate::{cache, EsedbInfo};

use super::Iter;

pub struct Table<'info, 'db>
where
    'info: 'db,
{
    table_id: &'static str,

    table: &'info libesedb::Table<'db>,

    columns: Rc<Vec<cache::Column>>,

    records: Vec<cache::Record<'info, 'db>>,
}

impl<'info, 'db> Table<'info, 'db>
where
    'info: 'db,
{
    pub fn try_from(
        table: &'info libesedb::Table<'db>,
        table_id: &'static str,
        esedbinfo: &'info EsedbInfo<'db>,
    ) -> std::io::Result<Self> {
        let mut columns = Vec::new();

        for column in table.iter_columns()? {
            columns.push(cache::Column::try_from(column?)?);
        }
        let columns = Rc::new(columns);

        let mut records = Vec::new();
        for (mut record_id, record) in table.iter_records().unwrap().enumerate() {
            records.push(
                cache::Record::try_from(
                    record.unwrap(),
                    table_id,
                    record_id as i32,
                    esedbinfo,
                    Rc::clone(&columns),
                )
                .unwrap(),
            );
            record_id += 1;
        }

        Ok(Self {
            table,
            table_id,
            records,
            columns,
        })
    }

    pub fn iter(&'db self) -> Iter<'info, 'db> {
        self.records.iter().into()
    }

    pub fn count_columns(&self) -> i32 {
        self.columns.len().try_into().unwrap()
    }

    pub fn column(&self, pos: i32) -> Option<&cache::Column> {
        self.columns.get(usize::try_from(pos).unwrap())
    }
}
