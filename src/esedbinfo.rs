use getset::Getters;
use libesedb::{EseDb, Table};

use crate::ColumnInfoMapping;

//#[derive(Getters)]
//#[getset(get="pub")]
pub struct EsedbInfo<'db> {
    data_table: Table<'db>,
    link_table: Table<'db>,
    mapping: ColumnInfoMapping
}

impl<'db> TryFrom<&'db EseDb> for EsedbInfo<'db> {
    type Error = anyhow::Error;

    fn try_from(esedb: &'db EseDb) -> Result<Self, Self::Error> {
        let data_table = esedb.table_by_name("datatable")?;
        let link_table = esedb.table_by_name("link_table")?;
        let mapping = ColumnInfoMapping::try_from(&data_table)?;

        Ok(Self { data_table, link_table, mapping })
    }
}

impl<'db> EsedbInfo<'db> {
    pub fn mapping<'info> (&'info self) -> &'info ColumnInfoMapping where 'info: 'db {
        &self.mapping
    }

    pub fn data_table<'info>(&'info self) -> &Table<'db> where 'info: 'db {
        &self.data_table
    }

    pub fn link_table<'info>(&'info self) -> &Table<'db> where 'info: 'db {
        &self.link_table
    }
}
