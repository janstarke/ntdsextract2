use std::rc::Rc;

use crate::{
    cache::{self, MetaDataCache},
    ntds::{self, Computer, DataTable, Group, LinkTable, ObjectType, Person, Schema},
    object_tree_entry::ObjectTreeEntry,
    EsedbInfo, OutputOptions, SerializationType, EntryId, progress_bar::create_progressbar,
};

pub struct CDatabase<'info, 'db> {
    _esedbinfo: &'info EsedbInfo<'db>,
    data_table: DataTable<'info, 'db>,
    link_table: Rc<LinkTable>,
}

impl<'info, 'db> CDatabase<'info, 'db> {
    pub fn new(esedbinfo: &'info EsedbInfo<'db>) -> anyhow::Result<Self> {
        let metadata_cache = MetaDataCache::try_from(esedbinfo)?;

        let bar = create_progressbar("Loading data".to_string(), 5)?;

        bar.set_message("reading schema information and creating record cache");
        let object_tree = ObjectTreeEntry::from(&metadata_cache);
        bar.inc(1);

        let special_records = ObjectTreeEntry::get_special_records(Rc::clone(&object_tree))?;
        let schema_record_id = special_records.schema().record_ptr();
        log::debug!("found the schema record id is '{}'", schema_record_id);

        let schema = Schema::new(&metadata_cache, &special_records);

        bar.set_message("loading data table");
        let cached_data_table =
            cache::DataTable::new(esedbinfo.data_table(), "datatable", esedbinfo, metadata_cache)?;
        bar.inc(1);

        bar.set_message("loading link table");
        let cached_link_table =
            cache::LinkTable::try_from(esedbinfo.link_table(), "link_table", esedbinfo)?;
        bar.inc(1);

        bar.set_message("creating link table object");
        let link_table = Rc::new(LinkTable::new(
            cached_link_table,
            &cached_data_table,
            *schema_record_id
        )?);
        bar.inc(1);

        bar.set_message("creating data table object");
        let data_table = DataTable::new(
            cached_data_table,
            object_tree,
            *schema_record_id,
            Rc::clone(&link_table),
            schema
        )?;
        bar.inc(1);

        bar.finish_and_clear();
        Ok(Self {
            _esedbinfo: esedbinfo,
            link_table,
            data_table,
        })
    }

    pub fn show_users<T: SerializationType>(&self, options: &OutputOptions) -> anyhow::Result<()> {
        self.show_typed_objects::<Person<T>>(options, ObjectType::Person)
    }

    pub fn show_groups<T: SerializationType>(&self, options: &OutputOptions) -> anyhow::Result<()> {
        self.show_typed_objects::<Group<T>>(options, ObjectType::Group)
    }

    pub fn show_computers<T: SerializationType>(
        &self,
        options: &OutputOptions,
    ) -> anyhow::Result<()> {
        self.show_typed_objects::<Computer<T>>(options, ObjectType::Computer)
    }

    pub fn show_typed_objects<O: ntds::FromDataTable>(
        &self,
        options: &OutputOptions,
        object_type: ObjectType,
    ) -> anyhow::Result<()> {
        self.data_table
            .show_typed_objects::<O>(options, object_type)
    }

    pub fn show_type_names<T>(&self, options: &OutputOptions) -> anyhow::Result<()>
    where
        T: SerializationType,
    {
        self.data_table.show_type_names::<T>(options)
    }

    pub fn show_timeline(
        &self,
        options: &OutputOptions
    ) -> anyhow::Result<()> {
        self.data_table.show_timeline(options, &self.link_table)
    }

    pub fn show_entry(&self, entry_id: EntryId) -> crate::ntds::Result<()> {
        self.data_table.show_entry(entry_id)
    }

    pub fn show_tree(&self, max_depth: u8) -> crate::ntds::Result<()> {
        self.data_table.show_tree(max_depth)
    }

    pub fn search_entries(&self, regex: &str) -> anyhow::Result<()> {
        self.data_table.search_entries(regex)
    }
}
