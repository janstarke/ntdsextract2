use std::collections::{HashMap, HashSet};
use std::io::stdout;
use std::rc::Rc;

use crate::cache::{RecordPointer, SpecialRecords};
use crate::cli::output::Writer;
use crate::cli::{EntryFormat, OutputFormat, OutputOptions};
use crate::ntds::DataTableRecord;
use crate::ntds::FromDataTable;
use crate::ntds::LinkTable;
use crate::ntds::NtdsAttributeId;
use crate::ntds::Result;
use crate::object_tree::ObjectTree;
use crate::progress_bar::create_progressbar;
use crate::serialization::{CsvSerialization, SerializationType};
use crate::{cache, EntryId};
use crate::{ntds, FormattedValue};
use bodyfile::Bodyfile3Line;
use getset::Getters;
use maplit::hashset;
use regex::Regex;
use serde_json::json;

use super::{Computer, Group, ObjectType, Person, Schema};

/// wraps a ESEDB Table.
/// This class assumes the a NTDS datatable is being wrapped
#[derive(Getters)]
#[getset(get = "pub")]
pub struct DataTable<'info, 'db> {
    data_table: cache::DataTable<'info, 'db>,
    //database: Option<Weak<CDatabase<'r>>>,
    schema_record_id: RecordPointer,
    object_tree: Rc<ObjectTree>,
    link_table: Rc<LinkTable>,
    schema: Schema,
    special_records: SpecialRecords,
}

impl<'info, 'db> DataTable<'info, 'db> {
    /// create a new datatable wrapper
    pub fn new(
        data_table: cache::DataTable<'info, 'db>,
        object_tree: Rc<ObjectTree>,
        schema_record_id: RecordPointer,
        link_table: Rc<LinkTable>,
        schema: Schema,
        special_records: SpecialRecords,
    ) -> Result<Self> {
        Ok(Self {
            data_table,
            schema_record_id,
            object_tree,
            link_table,
            schema,
            special_records,
        })
    }

    fn find_type_record(
        &'db self,
        object_type: ObjectType,
    ) -> anyhow::Result<Option<DataTableRecord<'info, 'db>>> {
        let mut records = self.find_type_records(hashset! {object_type})?;
        Ok(records.remove(&object_type))
    }

    pub fn find_type_records(
        &'db self,
        mut types: HashSet<ObjectType>,
    ) -> anyhow::Result<HashMap<ObjectType, DataTableRecord<'info, 'db>>> {
        let mut type_records = HashMap::new();
        /*
        let children = self.data_table.children_of(self.schema_record_id);

        if !children.count() > 0 {
            return Err(anyhow::anyhow!(Error::SchemaRecordHasNoChildren));
        }
        */
        for dbrecord in self
            .data_table
            .metadata()
            .children_of(&self.schema_record_id)
        {
            let object_name2 = dbrecord.rdn().to_string();

            log::trace!("found a new type definition: '{}'", object_name2);

            if let Ok(object_type) = &object_name2[..].try_into() {
                if types.remove(object_type) {
                    log::debug!("found requested type definition for '{object_name2}'");
                    let data_record = self
                        .data_table()
                        .data_table_record_from(*dbrecord.record_ptr())
                        .unwrap();
                    type_records.insert(ObjectType::try_from(&object_name2[..])?, data_record);
                }
            }

            if types.is_empty() {
                break;
            }
        }
        log::info!("found {} type definitions", type_records.len());
        Ok(type_records)
    }

    pub fn show_users<T: SerializationType>(&self, options: &OutputOptions) -> anyhow::Result<()> {
        log::debug!("show_users()");
        self.show_typed_objects::<Person<T>>(options, ObjectType::Person)
    }

    pub fn show_groups<T: SerializationType>(&self, options: &OutputOptions) -> anyhow::Result<()> {
        log::debug!("show_groups()");
        self.show_typed_objects::<Group<T>>(options, ObjectType::Group)
    }

    pub fn show_computers<T: SerializationType>(
        &self,
        options: &OutputOptions,
    ) -> anyhow::Result<()> {
        log::debug!("show_computers()");
        self.show_typed_objects::<Computer<T>>(options, ObjectType::Computer)
    }

    pub fn show_type_names<T>(&self, options: &OutputOptions) -> anyhow::Result<()>
    where
        T: SerializationType,
    {
        let mut type_names = HashSet::new();
        for dbrecord in self
            .data_table()
            .metadata()
            .children_of(&self.schema_record_id)
        {
            let object_name2 = dbrecord.rdn().to_string();

            type_names.insert(object_name2);

            if type_names.is_empty() {
                break;
            }
        }
        let names = self
            .data_table()
            .metadata()
            .children_of(&self.schema_record_id)
            .map(|dbrecord| dbrecord.rdn().to_string());
        options.format().unwrap().write_typenames(names)
    }

    pub fn show_tree(&self, max_depth: u8) -> Result<()> {
        let tree = self.object_tree.to_termtree(max_depth);
        println!("{}", tree);
        Ok(())
    }

    pub fn show_entry(&self, entry_id: EntryId, entry_format: EntryFormat) -> Result<()> {
        let record = match entry_id {
            EntryId::Id(id) => self.data_table.metadata().record(&id),
            EntryId::Rid(rid) => self.data_table.metadata().entries_with_rid(rid).next(),
        };

        match record {
            None => println!("no matching object found"),
            Some(entry) => {
                let record = self
                    .data_table()
                    .data_table_record_from(*entry.record_ptr())?;

                match entry_format {
                    EntryFormat::Simple => {
                        let all_attributes = record.all_attributes();
                        let header_width = all_attributes
                            .keys()
                            .map(|k| {
                                let k: &'static str = k.into();
                                k.len()
                            })
                            .max()
                            .unwrap();
                        let mut sorted_ids: Vec<_> = all_attributes
                            .keys()
                            .map(|id| {
                                let s: &'static str = id.into();
                                (id, s)
                            })
                            .collect();
                        sorted_ids.sort_by(|lhs, rhs| lhs.1.cmp(rhs.1));

                        for header in sorted_ids {
                            let value = all_attributes.get(header.0).unwrap();
                            println!("{: <header_width$}: {}", header.1, value.value());
                        }
                    }
                    EntryFormat::Json => {
                        let _ = serde_json::to_writer_pretty(stdout(), &json!(record));
                    }
                    EntryFormat::Table => {
                        let mut table = term_table::Table::from(&record);

                        if let Some(size) = termsize::get() {
                            let attrib_size = 20;
                            let value_size = if size.cols > (attrib_size + 2) {
                                size.cols - (attrib_size + 2)
                            } else {
                                0
                            };
                            table.set_max_column_widths(vec![
                                (0, attrib_size.into()),
                                (1, value_size.into()),
                            ])
                        }
                        println!("{}", table.render())
                    }
                }
            }
        }
        Ok(())
    }

    pub fn search_entries(&self, regex: &str) -> anyhow::Result<()> {
        let re = Regex::new(regex)?;
        let mut table_columns = vec![
            NtdsAttributeId::DsRecordId,
            NtdsAttributeId::DsParentRecordId,
            NtdsAttributeId::AttCommonName,
            NtdsAttributeId::AttRdn,
            NtdsAttributeId::AttObjectCategory,
        ];

        let mut records = Vec::new();

        for record in self.data_table.iter() {
            let matching_columns = record
                .all_attributes()
                .iter()
                .filter(|(_, attribute)| re.is_match(attribute.value().value()))
                .map(|(id, attribute)| {
                    (
                        *id,
                        (
                            attribute.column().to_string(),
                            attribute.attribute().to_string(),
                            attribute.value().to_string(),
                        ),
                    )
                })
                .collect::<HashMap<NtdsAttributeId, (String, String, String)>>();
            if !matching_columns.is_empty() {
                for id in matching_columns.keys() {
                    if !table_columns.contains(id) {
                        table_columns.push(*id);
                    }
                }
                records.push(record);
            }
        }

        let mut csv_wtr = csv::Writer::from_writer(std::io::stdout());
        let empty_string = "".to_owned();
        csv_wtr.write_record(table_columns.iter().map(|c| {
            let s: &str = c.into();
            s
        }))?;
        for record in records.into_iter() {
            let all_attributes = record.all_attributes();
            csv_wtr.write_record(table_columns.iter().map(|a| {
                all_attributes
                    .get(a)
                    .map(|attribute| attribute.value().value())
                    .unwrap_or(&empty_string)
                    .replace('\n', "\\n")
                    .replace('\r', "\\r")
            }))?;
        }
        Ok(())
    }

    pub fn show_typed_objects<O: ntds::FromDataTable>(
        &self,
        options: &OutputOptions,
        object_type: ObjectType,
    ) -> anyhow::Result<()> {
        let type_record = self
            .find_type_record(object_type)?
            .unwrap_or_else(|| panic!("missing record for type '{object_type}'"));
        let type_record_id = type_record.ds_record_id()?;
        log::info!("found type record with id {type_record_id}");

        let mut csv_wtr = csv::WriterBuilder::new()
            .flexible(false)
            .from_writer(std::io::stdout());
        let bar = create_progressbar(
            format!("loading {object_type} records"),
            (self
                .data_table()
                .metadata()
                .entries_of_type(&type_record_id)
                .count())
            .try_into()?,
        )?;

        let mut records = Vec::new();

        for record in self
            .data_table()
            .metadata()
            .entries_of_type(&type_record_id)
            .map(|e| self.data_table().data_table_record_from(*e.record_ptr()))
        {
            let record = record?;
            let dn = if *options.include_dn() {
                match self.object_tree().dn_of(record.ptr()) {
                    Some(dn) => FormattedValue::Value(dn),
                    None => FormattedValue::NoValue,
                }
            } else {
                FormattedValue::Hide
            };

            let record = O::new(record, options, self, &self.link_table, dn)?;
            match options.format().unwrap() {
                OutputFormat::Csv => {
                    csv_wtr.serialize(record)?;
                    csv_wtr.flush()?;
                }
                OutputFormat::Json => {
                    records.push(record);
                }
                OutputFormat::JsonLines => {
                    println!("{}", serde_json::to_string(&record)?);
                }
            }
            bar.inc(1);
        }

        if options.format().unwrap() == OutputFormat::Json {
            println!("{}", serde_json::to_string_pretty(&records)?);
        }

        bar.finish_and_clear();
        drop(csv_wtr);

        Ok(())
    }

    fn timelines_from_supported_type(
        &self,
        record: DataTableRecord,
        record_type: &ObjectType,
        options: &OutputOptions,
        link_table: &LinkTable,
        distinguished_name: FormattedValue<String>,
    ) -> anyhow::Result<Vec<Bodyfile3Line>> {
        Ok(match record_type {
            ObjectType::Person => Vec::<Bodyfile3Line>::from(Person::<CsvSerialization>::new(
                record,
                options,
                self,
                link_table,
                distinguished_name,
            )?),
            ObjectType::Group => Vec::<Bodyfile3Line>::from(Group::<CsvSerialization>::new(
                record,
                options,
                self,
                link_table,
                distinguished_name,
            )?),
            ObjectType::Computer => Vec::<Bodyfile3Line>::from(Computer::<CsvSerialization>::new(
                record,
                options,
                self,
                link_table,
                distinguished_name,
            )?),
        })
    }

    fn show_timeline_for_records<'a>(
        &self,
        options: &OutputOptions,
        link_table: &LinkTable,
        records: impl Iterator<Item = &'a RecordPointer>,
    ) -> anyhow::Result<()> {
        let known_types: HashMap<_, _> = self
            .schema
            .supported_type_entries()
            .iter()
            .map(|(ot, ptr)| (ptr.ds_record_id(), ot))
            .collect();

        records
            .map(|ptr| &self.data_table().metadata()[ptr])
            .map(|e| self.data_table().data_table_record_from(*e.record_ptr()))
            .try_for_each(|r| {
                let record = r?;
                let lines = if let Some(object_type) = record.att_object_type_id_opt()? {
                    if let Some(record_type) = known_types.get(&object_type) {
                        self.timelines_from_supported_type(
                            record,
                            record_type,
                            options,
                            link_table,
                            FormattedValue::Hide,
                        )?
                    } else {
                        record.to_bodyfile(self.data_table().metadata())?
                    }
                } else {
                    record.to_bodyfile(self.data_table().metadata())?
                };

                for line in lines.into_iter() {
                    println!("{line}");
                }
                Ok(())
            })
    }

    pub fn show_timeline(
        &self,
        options: &OutputOptions,
        link_table: &LinkTable,
        include_deleted: bool,
    ) -> anyhow::Result<()> {
        let types = if *options.show_all_objects() {
            self.schema
                .all_type_entries()
                .iter()
                .map(|e| *e.ds_record_id())
                .collect()
        } else {
            self.schema
                .supported_type_entries()
                .values()
                .map(|e| *e.ds_record_id())
                .collect()
        };
        self.show_timeline_for_records(
            options,
            link_table,
            self.data_table()
                .metadata()
                .entries_of_types(types)
                .map(|e| e.record_ptr()),
        )?;

        if include_deleted {
            let deleted_objects_records: HashSet<_> = HashSet::from_iter(
                self.data_table()
                    .metadata()
                    .children_ptr_of(self.special_records().deleted_objects().record_ptr()),
            );

            let records_with_deleted_from_container_guid: HashSet<_> = HashSet::from_iter(
                self.data_table()
                    .metadata()
                    .entries_with_deleted_from_container_guid(),
            );
            let records = deleted_objects_records.union(&records_with_deleted_from_container_guid);

            self.show_timeline_for_records(options, link_table, records.copied())
                .unwrap();
        }

        Ok(())
    }
}
