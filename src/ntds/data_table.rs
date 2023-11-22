use std::collections::{HashMap, HashSet};
use std::rc::{Rc, Weak};

use crate::ntds::DataTableRecord;
use crate::object_tree_entry::ObjectTreeEntry;
use crate::{CDataTable, CRecord};
use crate::{CDatabase, CTable, RecordHasAttRdn, RecordHasId};
use anyhow::{bail, Result};
use maplit::hashset;

use super::WriteTypenames;

/// wraps a ESEDB Table.
/// This class assumes the a NTDS datatable is being wrapped
pub struct DataTable<'d> {
    data_table: CDataTable<'d>,
    database: Option<Weak<CDatabase<'d>>>,
    schema_record_id: i32,
    object_tree: Rc<ObjectTreeEntry>,
}

impl<'d> DataTable<'d> {
    /// create a new datatable wrapper
    pub fn new<'l>(
        data_table: CDataTable<'d>,
        object_tree: Rc<ObjectTreeEntry>,
        schema_record_id: i32,
    ) -> Result<Self> {
        Ok(Self {
            database: None,
            data_table,
            schema_record_id,
            object_tree,
        })
    }

    /// returns the record id of the record which contains the Schema object
    /// (which is identified by its name "Schema" in the object_name2 attribute)
    pub fn get_schema_record_id(data_table: &CTable) -> anyhow::Result<i32> {
        log::info!("obtaining schema record id");

        for record in data_table
            .filter_p(RecordHasAttRdn("Schema"))
            .map(DataTableRecord::from)
        {
            if let Some(schema_parent_id) = record.ds_parent_record_id_opt()? {
                if let Some(schema_parent) = data_table
                    .find_p(RecordHasId(schema_parent_id))
                    .map(DataTableRecord::from)
                {
                    if let Some(parent_name) = schema_parent.ds_object_name2_opt()? {
                        if parent_name == "Configuration" {
                            log::info!("found record id to be {}", record.ds_record_id()?);
                            return Ok(record.ds_record_id()?);
                        }
                    }
                }
            }
        }
        bail!("no schema record found");
    }

    pub fn set_database(&mut self, database: Weak<CDatabase<'d>>) {
        self.database = Some(database);
    }

    pub fn database(&self) -> &CDatabase {
        &self.database.unwrap().upgrade().unwrap()
    }

    pub(crate) fn data_table(&self) -> &CDataTable {
        &self.data_table
    }

    fn find_type_record(&self, type_name: &str) -> Result<Option<DataTableRecord>> {
        let mut records = self.find_type_records(hashset! {type_name})?;
        Ok(records.remove(type_name))
    }

    pub fn find_all_type_names(&self) -> Result<HashMap<i32, String>> {
        let mut type_records = HashMap::new();
        for dbrecord in self.data_table.children_of(self.schema_record_id) {
            let object_name2 = dbrecord.ds_object_name2()?.to_string();

            type_records.insert(dbrecord.ds_record_id()?, object_name2);
        }
        log::info!("found all required type definitions");
        Ok(type_records)
    }

    pub fn find_type_records(
        &self,
        mut type_names: HashSet<&str>,
    ) -> Result<HashMap<String, DataTableRecord>> {
        let mut type_records = HashMap::new();
        let children = self.data_table.children_of(self.schema_record_id);
        anyhow::ensure!(children.count() > 0, "The schema record has no children");

        for dbrecord in self.data_table.children_of(self.schema_record_id) {
            let object_name2 = dbrecord.ds_object_name2()?.to_string();

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
    /*
       pub fn show_users(&self, format: &OutputFormat) -> Result<()> {
           self.show_typed_objects::<Person>(format, TYPENAME_PERSON)
       }

       pub fn show_groups(&self, format: &OutputFormat) -> Result<()> {
           self.show_typed_objects::<Group>(format, TYPENAME_GROUP)
       }

       pub fn show_computers(&self, format: &OutputFormat) -> Result<()> {
           self.show_typed_objects::<Computer>(format, TYPENAME_COMPUTER)
       }
    */
    pub fn show_type_names(&self, writer: &impl WriteTypenames) -> Result<()> {
        let mut type_names = HashSet::new();
        for dbrecord in self
            .data_table
            .children_of(self.schema_record_id)
        {
            let object_name2 = dbrecord
                .ds_object_name2()?;

            type_names.insert(object_name2);

            if type_names.is_empty() {
                break;
            }
        }
        let x = self
            .data_table
            .children_of(self.schema_record_id)
            .map(|dbrecord| dbrecord.ds_object_name2())
            .filter_map(Result::ok);
        writer.write_typenames(x);
        /*

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
        */
        Ok(())
    }
    /*
       pub fn show_tree(&self, max_depth: u8) -> Result<()> {
           let tree = ObjectTreeEntry::to_tree(&self.object_tree, max_depth);
           println!("{}", tree);
           Ok(())
       }

       pub fn show_entry(&self, entry_id: EntryId) -> Result<()> {
           let mapping = &self.mapping;

           let record = match entry_id {
               EntryId::Id(id) => self.data_table.find_by_id(mapping, id),
               EntryId::Rid(rid) => self.data_table.find_by_rid(mapping, rid),
           };

           match record {
               None => println!("no matching object found"),
               Some(entry) => {
                   let mut table = entry.to_table(&self.mapping);

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
           Ok(())
       }

       pub fn search_entries(&self, regex: &str) -> Result<()> {
           let mapping = &self.mapping;
           let re = Regex::new(regex)?;
           let mut table_columns = vec![
               "DNT_col".to_owned(),
               "PDNT_col".to_owned(),
               "ATTm3".to_owned(),
               "ATTm589825".to_owned(),
               "ATTb590606".to_owned(),
           ];

           let mut records = Vec::new();

           for record in self.data_table.iter_records() {
               let matching_columns = record
                   .all_attributes(mapping)
                   .iter()
                   .filter(|(_, v)| re.is_match(v))
                   .map(|(a, v)| (a.to_owned(), v.to_owned()))
                   .collect::<Vec<(String, String)>>();
               if !matching_columns.is_empty() {
                   for (a, _) in matching_columns {
                       if !table_columns.contains(&a) {
                           table_columns.push(a);
                       }
                   }
                   records.push(record);
               }
           }

           let mut csv_wtr = csv::Writer::from_writer(std::io::stdout());
           let empty_string = "".to_owned();
           csv_wtr.write_record(&table_columns)?;
           for record in records.into_iter() {
               let all_attributes = record.all_attributes(mapping);
               csv_wtr.write_record(table_columns.iter().map(|a| {
                   all_attributes
                       .get(a)
                       .unwrap_or(&empty_string)
                       .replace('\n', "\\n")
                       .replace('\r', "\\r")
               }))?;
           }
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

           for record in self
               .data_table
               .iter_records()
               .filter(|dbrecord| dbrecord.ds_object_type_id(&self.mapping).is_ok())
               .filter(|dbrecord| dbrecord.ds_object_type_id(&self.mapping).unwrap() == type_record_id)
               .map(|dbrecord| T::from(dbrecord, self.database()).unwrap())
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
               Some(
                   type_records
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
                       .collect::<HashMap<i32, &String>>(),
               )
           };

           for bf_lines in self
               .data_table
               .iter_records()
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
                                   match <Person as FromDbRecord>::from(dbrecord, self.database()) {
                                       Ok(person) => Some(Vec::<Bodyfile3Line>::from(person)),
                                       Err(why) => {
                                           log::error!("unable to parse person: {why}");
                                           None
                                       }
                                   }
                               } else if *type_name == TYPENAME_COMPUTER {
                                   match <Computer as FromDbRecord>::from(dbrecord, self.database()) {
                                       Ok(computer) => Some(Vec::<Bodyfile3Line>::from(computer)),
                                       Err(why) => {
                                           log::error!("unable to parse person: {why}");
                                           None
                                       }
                                   }
                               } else {
                                   None
                               }
                           }
                           _ => None,
                       }
                   } else {
                       Some(
                           dbrecord
                               .to_bodyfile(&self.mapping, &all_type_records[&current_type_id][..])
                               .expect("unable to create timeline from DbRecord"),
                       )
                   }
               })
               .flatten()
           {
               println!("{}", bf_lines)
           }
           Ok(())
       }
    */
}
