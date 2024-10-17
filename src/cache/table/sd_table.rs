use std::rc::Rc;

use getset::Getters;

use crate::{
    cache::{self, ColumnIndex, ColumnsOfTable},
    EsedbInfo,
};

#[derive(Getters)]
pub struct SdTable<'info, 'db>
where
    'info: 'db,
{
    _table_id: &'static str,
    _table: &'info libesedb::Table<'db>,
    esedbinfo: &'info EsedbInfo<'db>,

    #[getset(get = "pub")]
    sd_id_column: ColumnIndex,

    #[getset(get = "pub")]
    sd_hash_column: ColumnIndex,

    #[getset(get = "pub")]
    sd_refcount_column: ColumnIndex,

    #[getset(get = "pub")]
    sd_value_column: ColumnIndex,

    // this is needed for `::all_atributes`
    columns: Rc<ColumnsOfTable>,
}

impl<'info, 'db> SdTable<'info, 'db>
where
    'info: 'db,
{
    pub fn try_from(
        table_id: &'static str,
        esedbinfo: &'info EsedbInfo<'db>,
    ) -> std::io::Result<Self> {
        let table = esedbinfo.sd_table();
        let columns = ColumnsOfTable::try_from(table)?;

        Ok(Self {
            _table: table,
            _table_id: table_id,
            esedbinfo,
            sd_id_column: *columns["sd_id"].index(),
            sd_hash_column: *columns["sd_hash"].index(),
            sd_refcount_column: *columns["sd_refcount"].index(),
            sd_value_column: *columns["sd_value"].index(),
            columns: Rc::new(ColumnsOfTable::try_from(table)?)
        })
    }
}

impl<'info, 'db> SdTable<'info, 'db> {
    pub fn iter(&'db self) -> impl Iterator<Item = cache::Record<'info, 'db>> {
        self._table
            .iter_records()
            .unwrap()
            .map(|r| r.unwrap())
            .zip(0..)
            .map(|(r, row)| {
                cache::Record::try_from(
                    r,
                    self._table_id,
                    row.into(),
                    self.esedbinfo,
                    Rc::clone(&self.columns),
                )
                .unwrap()
            })
    }
}
