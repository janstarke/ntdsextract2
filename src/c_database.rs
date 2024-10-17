use std::rc::Rc;

use crate::{
    cache::{self, MetaDataCache},
    cli::{EntryFormat, OutputOptions, TimelineFormat},
    ntds::{self, Computer, DataTable, Group, LinkTable, ObjectType, Person, Schema, SdTable},
    object_tree::ObjectTree,
    EntryId, EsedbInfo, SerializationType,
};

pub struct CDatabase<'info, 'db> {
    _esedbinfo: &'info EsedbInfo<'db>,
    data_table: DataTable<'info, 'db>,
    link_table: Rc<LinkTable>,
    _sd_table: Rc<SdTable>
}

impl<'info, 'db> CDatabase<'info, 'db> {
    pub fn new(esedbinfo: &'info EsedbInfo<'db>) -> anyhow::Result<Self> {
        let cached_sd_table = cache::SdTable::try_from("sd_table", esedbinfo)?;
        let sd_table = Rc::new(SdTable::new(&cached_sd_table)?);

        let metadata_cache = MetaDataCache::try_from(esedbinfo)?;

        let object_tree = Rc::new(ObjectTree::new(&metadata_cache, Rc::clone(&sd_table)));

        let special_records = object_tree.get_special_records()?;
        let schema_record_id = special_records.schema().record_ptr();
        log::debug!("found the schema record id is '{}'", schema_record_id);

        let schema = Schema::new(&metadata_cache, &special_records);

        let cached_data_table = cache::DataTable::new(
            esedbinfo.data_table(),
            "datatable",
            esedbinfo,
            metadata_cache,
        )?;

        let cached_link_table =
            cache::LinkTable::try_from(esedbinfo.link_table(), "link_table", esedbinfo)?;

        let link_table = Rc::new(LinkTable::new(
            cached_link_table,
            &cached_data_table,
            *schema_record_id,
        )?);

        let data_table = DataTable::new(
            cached_data_table,
            object_tree,
            *schema_record_id,
            Rc::clone(&link_table),
            Rc::clone(&sd_table),
            schema,
            special_records,
        )?;


        Ok(Self {
            _esedbinfo: esedbinfo,
            link_table,
            data_table,
            _sd_table: sd_table,
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

    pub fn show_typed_objects<O: ntds::FromDataTable + ntds::IsMemberOf>(
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
        options: &OutputOptions,
        include_deleted: bool,
        format: &TimelineFormat,
    ) -> anyhow::Result<()> {
        self.data_table
            .show_timeline(options, &self.link_table, include_deleted, format)
    }

    pub fn show_entry(
        &self,
        entry_id: EntryId,
        entry_format: EntryFormat,
    ) -> crate::ntds::Result<()> {
        self.data_table.show_entry(entry_id, entry_format)
    }

    pub fn show_tree(&self, max_depth: u8) -> crate::ntds::Result<()> {
        self.data_table.show_tree(max_depth)
    }

    pub fn search_entries(&self, regex: &str) -> anyhow::Result<()> {
        self.data_table.search_entries(regex)
    }
}
