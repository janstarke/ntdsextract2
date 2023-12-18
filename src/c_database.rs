use std::rc::Rc;

use getset::Getters;

use crate::{
    cache,
    ntds::{DataTable, LinkTable},
    object_tree_entry::ObjectTreeEntry,
    EsedbInfo,
};

#[derive(Getters)]
#[getset(get="pub")]
pub struct CDatabase<'info, 'db> {
    esedbinfo: &'info EsedbInfo<'db>,
    data_table: DataTable<'info, 'db>,
    link_table: Rc<LinkTable>,
}

impl<'info, 'db> CDatabase<'info, 'db> {
    pub fn new(esedbinfo: &'info EsedbInfo<'db>) -> anyhow::Result<Self> {
        let cached_data_table =
            cache::Table::try_from(esedbinfo.data_table(), "datatable", esedbinfo)?;
        log::info!("cached data_table");

        let cached_link_table =
            cache::Table::try_from(esedbinfo.link_table(), "link_table", esedbinfo)?;
        log::info!("cached link_table");

        log::info!("reading schema information and creating record cache");
        let object_tree = ObjectTreeEntry::from(&cached_data_table)?;
        let special_records = cached_data_table.get_special_records(Rc::clone(&object_tree))?;
        let schema_record_id = special_records.schema().id();

        log::debug!("found the schema record id is '{}'", schema_record_id);

        let link_table = Rc::new(LinkTable::new(cached_link_table, &cached_data_table, *schema_record_id)?);
        let data_table = DataTable::new(cached_data_table, object_tree, *schema_record_id, Rc::clone(&link_table))?;
        Ok(Self {
            esedbinfo,
            link_table,
            data_table,
        })
    }
}
