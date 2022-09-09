use std::collections::{HashMap, HashSet};

use anyhow::Result;
use bodyfile::Bodyfile3Line;
use libesedb::Table;
use maplit::hashset;
use serde::Serialize;
use crate::computer::Computer;
use crate::constants::*;
use crate::{DbRecord, FromDbRecord};

use crate::{
    column_info_mapping::ColumnInfoMapping, person::Person, OutputFormat, constants::TYPENAME_COMPUTER,
};

pub(crate) struct DataTableExt<'a> {
    data_table: Table<'a>,
    mapping: ColumnInfoMapping,
    schema_record_id: i32,
}

impl<'a> DataTableExt<'a> {
    pub fn from(table: Table<'a>) -> Result<Self> {
        log::info!("reading schema information and creating record cache");
        let mapping = ColumnInfoMapping::from(&table)?;
        let schema_record_id = Self::get_schema_record_id(&table, &mapping)?;
        Ok(Self {
            data_table: table,
            mapping,
            schema_record_id,
        })
    }

    fn get_schema_record_id(data_table: &Table<'a>, mapping: &ColumnInfoMapping) -> Result<i32> {
        let schema_record = data_table
            .iter_records()?
            .filter_map(|r| r.ok())
            .map(DbRecord::from)
            .find(|dbrecord| {
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

    pub fn find_type_records(
        &self,
        mut type_names: HashSet<&str>,
    ) -> Result<HashMap<String, DbRecord>> {
        let mut type_records = HashMap::new();
        for dbrecord in self
            .data_table
            .iter_records()?
            .filter_map(|r| r.ok())
            .map(DbRecord::from)
            .filter(|dbrecord| {
                dbrecord
                    .ds_parent_record_id(&self.mapping)
                    .unwrap()
                    .unwrap()
                    == self.schema_record_id
            })
        {
            let object_name2 = dbrecord
                .ds_object_name2(&self.mapping)?
                .expect("missing object_name2 attribute");

            if type_names.remove(&object_name2[..]) {
                log::trace!("found type definition for '{object_name2}'");
                type_records.insert(object_name2, dbrecord);
            }

            if type_names.is_empty() {
                break;
            }
        }
        log::info!("found all required type definitions");
        Ok(type_records)
    }

    pub fn show_users(&self, format: &OutputFormat) -> Result<()> {
        self.show_typed_objects::<Person>(format, TYPENAME_PERSON)
    }

    pub fn show_computers(&self, format: &OutputFormat) -> Result<()> {
        self.show_typed_objects::<Computer>(format, TYPENAME_COMPUTER)
    }

    fn show_typed_objects<T: FromDbRecord + Serialize>(&self, format: &OutputFormat, type_name: &str) -> Result<()> {
        let type_record = self
            .find_type_record(type_name)?
            .unwrap_or_else(|| panic!("missing record for type '{}'", type_name));
        let type_record_id = type_record.ds_record_id(&self.mapping)?;

        let mut csv_wtr = csv::Writer::from_writer(std::io::stdout());
        
        for record in self
            .data_table
            .iter_records()?
            .filter_map(|r| r.ok())
            .map(DbRecord::from)
            .filter(|dbrecord| dbrecord.ds_object_type_id(&self.mapping).is_ok())
            .filter(|dbrecord| {
                dbrecord.ds_object_type_id(&self.mapping).unwrap() == type_record_id
            })
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

    pub fn show_timeline(&self) -> Result<()> {
        let type_records = self.find_type_records(hashset! {
            TYPENAME_PERSON,
            TYPENAME_COMPUTER
        })?;
        let type_record_ids = type_records
            .iter()
            .map(|(type_name, dbrecord)| {
                (
                    dbrecord
                        .ds_record_id(&self.mapping)
                        .expect("unable to read record id")
                        .expect("missing record id"),
                    type_name,
                )
            })
            .collect::<HashMap<i32, &String>>();

        for bf_lines in self
            .data_table
            .iter_records()?
            .filter_map(|r| r.ok())
            .map(DbRecord::from)
            .filter(|dbrecord| dbrecord.ds_object_type_id(&self.mapping).is_ok())
            .filter(|dbrecord| dbrecord.ds_object_type_id(&self.mapping).unwrap().is_some())
            .filter_map(|dbrecord| {
                let current_type_id = dbrecord
                    .ds_object_type_id(&self.mapping)
                    .unwrap()
                    .expect("missing object type id");
                match type_record_ids.get(&current_type_id) {
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
            })
            .flatten()
        {
            println!("{}", bf_lines)
        }
        Ok(())
    }
}
