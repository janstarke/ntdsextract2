use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::{Rc, Weak};

use crate::column_info_mapping::RecordToBodyfile;
use crate::computer::Computer;
use crate::constants::*;
use crate::esedb_utils::*;
use crate::group::Group;
use crate::link_table_ext::LinkTableExt;
use crate::object_tree_entry::ObjectTreeEntry;
use crate::{DbRecord, FromDbRecord};
use anyhow::{Result, bail};
use bodyfile::Bodyfile3Line;
use libesedb::Table;
use maplit::hashset;
use serde::Serialize;
use termtree::Tree;

use crate::{
    column_info_mapping::ColumnInfoMapping, constants::TYPENAME_COMPUTER, person::Person,
    OutputFormat,
};

/// wraps a ESEDB Table.
/// This class assumes the a NTDS datatable is being wrapped
pub(crate) struct DataTableExt<'a> {
    data_table: Table<'a>,
    link_table: LinkTableExt,
    mapping: ColumnInfoMapping,
    schema_record_id: i32,
    object_tree: Rc<ObjectTreeEntry>,
}

impl<'a> DataTableExt<'a> {
    /// create a new datatable wrapper
    pub fn from(table: Table<'a>, link_table: LinkTableExt) -> Result<Self> {
        log::info!("reading schema information and creating record cache");
        let mapping = ColumnInfoMapping::from(&table)?;
        let object_tree = ObjectTreeEntry::from(&table, &mapping)?;
        let schema_record_id = Self::get_schema_record_id(&table, &mapping)?;
        log::debug!("found the schema record id is '{}'", schema_record_id);
        Ok(Self {
            data_table: table,
            link_table,
            mapping,
            schema_record_id,
            object_tree
        })
    }

    fn find_children_of<'b>(&'a self, parent_id: i32) -> Box<dyn Iterator<Item = DbRecord<'b>> + 'b>
    where
        'a: 'b,
    {
        let data_table = &self.data_table;
        let mapping = &self.mapping;

        log::debug!("searching for children of record '{}'", parent_id);

        Box::new(filter_records_from(
            data_table,
            move |dbrecord: &DbRecord| {
                dbrecord.ds_parent_record_id(mapping).unwrap().unwrap() == parent_id
            },
        ))
    }

    /// returns the record id of the record which contains the Schema object
    /// (which is identified by its name "Schema" in the object_name2 attribute)
    fn get_schema_record_id(data_table: &Table<'a>, mapping: &ColumnInfoMapping) -> Result<i32> {
        let schema_record = find_record_from(data_table, |dbrecord| {
            "Schema"
                == dbrecord
                    .ds_object_name2(mapping)
                    .expect("unable to read object_name2 attribute")
                    .expect("missing object_name2 attribute")
        })
        .expect("no schema record found");
        let schema_record_id = schema_record
            .ds_record_id(mapping)?
            .expect("Schema record has no record ID");
        Ok(schema_record_id)
    }

    fn find_type_record(&self, type_name: &str) -> Result<Option<DbRecord>> {
        let mut records = self.find_type_records(hashset! {type_name})?;
        Ok(records.remove(type_name))
    }

    pub fn find_all_type_names(&self) -> Result<HashMap<i32, String>> {
        let mut type_records = HashMap::new();
        for dbrecord in self.find_children_of(self.schema_record_id) {
            let object_name2 = dbrecord
                .ds_object_name2(&self.mapping)?
                .expect("missing object_name2 attribute");

                type_records.insert(dbrecord.ds_record_id(&self.mapping)?.unwrap(), object_name2);
        }
        log::info!("found all required type definitions");
        Ok(type_records)
    }

    pub fn find_type_records(
        &self,
        mut type_names: HashSet<&str>,
    ) -> Result<HashMap<String, DbRecord>> {
        let mut type_records = HashMap::new();
        let children = self.find_children_of(self.schema_record_id);
        anyhow::ensure!(children.count() > 0, "The schema record has no children");

        for dbrecord in self.find_children_of(self.schema_record_id) {
            let object_name2 = dbrecord
                .ds_object_name2(&self.mapping)?
                .expect("missing object_name2 attribute");
            
            log::trace!("found a new type definition: '{}'", object_name2);

            if type_names.remove(&object_name2[..]) {
                log::debug!("found requested type definition for '{object_name2}'");
                type_records.insert(object_name2, dbrecord);
            }

            if type_names.is_empty() {
                break;
            }
        }
        log::info!("found {} type definitions", type_records.len());
        Ok(type_records)
    }

    pub fn show_users(&self, format: &OutputFormat) -> Result<()> {
        self.show_typed_objects::<Person>(format, TYPENAME_PERSON)
    }

    pub fn show_groups(&self, format: &OutputFormat) -> Result<()> {
        self.show_typed_objects::<Group>(format, TYPENAME_GROUP)
    }

    pub fn show_computers(&self, format: &OutputFormat) -> Result<()> {
        self.show_typed_objects::<Computer>(format, TYPENAME_COMPUTER)
    }

    pub fn show_type_names(&self, format: &OutputFormat) -> Result<()> {
        let mut type_names = HashSet::new();
        for dbrecord in self.find_children_of(self.schema_record_id) {
            let object_name2 = dbrecord
                .ds_object_name2(&self.mapping)?
                .expect("missing object_name2 attribute");

            type_names.insert(object_name2);

            if type_names.is_empty() {
                break;
            }
        }

        match format {
            OutputFormat::Csv => {
                let mut csv_wtr = csv::Writer::from_writer(std::io::stdout());
                csv_wtr.serialize(type_names)?
            }
            OutputFormat::Json => {
                println!("{}", serde_json::to_string_pretty(&type_names)?);
            }
            OutputFormat::JsonLines => {
                println!("{}", serde_json::to_string(&type_names)?);
            }
        }

        Ok(())
    }

    pub fn show_tree(&self, max_depth: u8) -> Result<()> {
        let tree = ObjectTreeEntry::to_tree(&self.object_tree, max_depth);
        println!("{}", tree);
        Ok(())
    }

    fn show_typed_objects<T: FromDbRecord + Serialize>(
        &self,
        format: &OutputFormat,
        type_name: &str,
    ) -> Result<()> {
        let type_record = self
            .find_type_record(type_name)?
            .unwrap_or_else(|| panic!("missing record for type '{}'", type_name));
        let type_record_id = type_record.ds_record_id(&self.mapping)?;

        let mut csv_wtr = csv::Writer::from_writer(std::io::stdout());

        for record in iter_records(&self.data_table)
            .filter(|dbrecord| dbrecord.ds_object_type_id(&self.mapping).is_ok())
            .filter(|dbrecord| dbrecord.ds_object_type_id(&self.mapping).unwrap() == type_record_id)
            .map(|dbrecord| T::from(dbrecord, &self.mapping).unwrap())
        {
            match format {
                OutputFormat::Csv => {
                    csv_wtr.serialize(record)?;
                }
                OutputFormat::Json => {
                    println!("{}", serde_json::to_string_pretty(&record)?);
                }
                OutputFormat::JsonLines => {
                    println!("{}", serde_json::to_string(&record)?);
                }
            }
        }
        drop(csv_wtr);

        Ok(())
    }

    pub fn show_timeline(&self, all_objects: bool) -> Result<()> {
        let type_records = self.find_type_records(hashset! {
            TYPENAME_PERSON,
            TYPENAME_COMPUTER})?;
        
        let all_type_records = self.find_all_type_names()?;

        let type_record_ids = if all_objects {
            None
        } else { 
            Some(type_records.iter()
            .map(|(type_name, dbrecord)| {
                (
                    dbrecord
                        .ds_record_id(&self.mapping)
                        .expect("unable to read record id")
                        .expect("missing record id"),
                    type_name,
                )
            })
            .collect::<HashMap<i32, &String>>())
        };

        for bf_lines in iter_records(&self.data_table)
            .filter(|dbrecord| dbrecord.has_valid_ds_object_type_id(&self.mapping))
            .filter_map(|dbrecord| {
                let current_type_id = dbrecord
                    .ds_object_type_id(&self.mapping)
                    .unwrap()
                    .expect("missing object type id");
                
                // `type_record_ids` is None if `all_objects` is True
                if let Some(record_ids) = type_record_ids.as_ref() {
                    match record_ids.get(&current_type_id) {
                        Some(type_name) => {
                            if *type_name == TYPENAME_PERSON {
                                Some(Vec::<Bodyfile3Line>::from(
                                    <Person as FromDbRecord>::from(dbrecord, &self.mapping).unwrap(),
                                ))
                            } else if *type_name == TYPENAME_COMPUTER {
                                Some(Vec::<Bodyfile3Line>::from(
                                    <Computer as FromDbRecord>::from(dbrecord, &self.mapping).unwrap(),
                                ))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                } else {
                    Some(dbrecord.to_bodyfile(&self.mapping, &all_type_records[&current_type_id][..]).expect("unable to create timeline from DbRecord"))
                }
            })
            .flatten()
        {
            println!("{}", bf_lines)
        }
        Ok(())
    }
}
