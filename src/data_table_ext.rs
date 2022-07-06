use std::collections::{HashSet, HashMap};

use anyhow::Result;
use libesedb::Table;
use maplit::hashset;

use crate::{column_info_mapping::ColumnInfoMapping, dbrecord::DbRecord, OutputFormat, person::Person};

pub (crate) struct DataTableExt<'a> {
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
            schema_record_id
        })
    }


    fn get_schema_record_id(data_table: &Table<'a>, mapping: &ColumnInfoMapping) -> Result<i32> {
        let schema_record = data_table
            .iter_records()?
            .filter_map(|r| r.ok())
            .map(|r| DbRecord::from(r))
            .find(|dbrecord| {
                "Schema"
                    == dbrecord
                        .ds_object_name2_index(&mapping)
                        .expect("unable to read object_name2 attribute")
                        .expect("missing object_name2 attribute")
            })
            .expect("no schema record found");
        let schema_record_id = schema_record.ds_record_id_index(&mapping)?.expect("Schema record has no record ID");
        Ok(schema_record_id)
    }

    fn find_type_record(&self, type_name: &str) -> Result<Option<DbRecord>> {
        let mut records = self.find_type_records(hashset!{type_name})?;
        Ok(records.remove(type_name))
    }

    pub fn find_type_records(&self, mut type_names: HashSet<&str>) -> Result<HashMap<String, DbRecord>> {
        let mut type_records = HashMap::new();
        for dbrecord in self.data_table.iter_records()?
                                    .filter_map(|r| r.ok()) 
                                    .map(|r| DbRecord::from(r))
                                    .filter(|dbrecord| dbrecord.ds_parent_record_id_index(&self.mapping).unwrap().unwrap() == self.schema_record_id) {
                    
            let object_name2 = dbrecord.ds_object_name2_index(&self.mapping)?.expect("missing object_name2 attribute");

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

    pub fn show_users(&self, _format: &OutputFormat) -> Result<()> {
        let type_record = self.find_type_record("Person")?.expect("missing record for type 'Person'");
        let type_record_id = type_record.ds_record_id_index(&self.mapping)?;

        let mut wtr = csv::Writer::from_writer(std::io::stdout());
        for person in self.data_table.iter_records()?
                                        .filter_map(|r| r.ok())
                                        .map(|r| DbRecord::from(r))
                                        .filter(|dbrecord| dbrecord.ds_object_type_id_index(&self.mapping).is_ok())
                                        .filter(|dbrecord| dbrecord.ds_object_type_id_index(&self.mapping).unwrap() == type_record_id)
                                        .map(|dbrecord| Person::from(dbrecord, &self.mapping).unwrap()){
            wtr.serialize(person)?;
        }
        drop(wtr);

        Ok(())
    }

    pub fn show_computers(&self, _format: &OutputFormat) -> Result<()> {
        todo!()
    }

    pub fn show_timeline(&self) -> Result<()> {
        todo!()
    }
}
