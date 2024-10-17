use getset::Getters;
use libesedb::{EseDb, Table};
use std::ops::Index;

use crate::{ntds::NtdsAttributeId, ColumnInfoMapping, ColumnInformation};

#[derive(Getters)]
#[getset(get="pub")]
pub struct EsedbInfo<'db> {
    data_table: Table<'db>,
    link_table: Table<'db>,
    sd_table: Table<'db>,
    mapping: ColumnInfoMapping,
}

impl<'db> TryFrom<&'db EseDb> for EsedbInfo<'db> {
    type Error = anyhow::Error;

    fn try_from(esedb: &'db EseDb) -> Result<Self, Self::Error> {
        let data_table = esedb.table_by_name("datatable")?;
        let link_table = esedb.table_by_name("link_table")?;
        let sd_table = esedb.table_by_name("sd_table")?;
        let mapping = ColumnInfoMapping::try_from(&data_table)?;

        Ok(Self {
            data_table,
            link_table,
            sd_table,
            mapping,
        })
    }
}

impl<'db> EsedbInfo<'db> {
    pub fn column(&self, att_id: NtdsAttributeId) -> &ColumnInformation {
        self.mapping().index(att_id)
    }
}
