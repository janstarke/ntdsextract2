use std::rc::Rc;

use getset::Getters;

use crate::{
    cache::{self, ColumnIndex, ColumnsOfTable},
    EsedbInfo,
};

#[derive(Getters)]
pub struct LinkTable<'info, 'db>
where
    'info: 'db,
{
    _table_id: &'static str,
    _table: &'info libesedb::Table<'db>,
    esedbinfo: &'info EsedbInfo<'db>,

    #[getset(get = "pub")]
    link_dnt_id: ColumnIndex,

    #[getset(get = "pub")]
    backlink_dnt_id: ColumnIndex,

    #[getset(get = "pub")]
    link_base_id: ColumnIndex,

    // this is needed for `::all_atributes`
    columns: Rc<ColumnsOfTable>,
}

impl<'info, 'db> LinkTable<'info, 'db>
where
    'info: 'db,
{
    pub fn try_from(
        table: &'info libesedb::Table<'db>,
        table_id: &'static str,
        esedbinfo: &'info EsedbInfo<'db>,
    ) -> std::io::Result<Self> {
        let columns = ColumnsOfTable::try_from(table)?;

        Ok(Self {
            _table: table,
            _table_id: table_id,
            esedbinfo,
            link_dnt_id: *columns["link_DNT"].index(),
            backlink_dnt_id: *columns["backlink_DNT"].index(),
            link_base_id: *columns["link_base"].index(),
            columns: Rc::new(ColumnsOfTable::try_from(table)?)
        })
    }
}

impl<'info, 'db> LinkTable<'info, 'db> {
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
