use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::ntds;
use crate::ntds::object::Object;
use crate::ntds::LinkTable;
use crate::ntds::Result;
use crate::ntds::{DataTableRecord, Error};
use crate::object_tree_entry::ObjectTreeEntry;
use crate::{cache, EntryId, OutputFormat, OutputOptions, RecordHasId, RecordHasRid};
use bodyfile::Bodyfile3Line;
use getset::Getters;
use maplit::hashset;
use regex::Regex;

use super::{Computer, ObjectType, Person, WriteTypenames};

/// wraps a ESEDB Table.
/// This class assumes the a NTDS datatable is being wrapped
#[derive(Getters)]
#[getset(get="pub")]
pub struct DataTable<'info, 'db> {
    data_table: cache::Table<'info, 'db, cache::DataTable>,
    //database: Option<Weak<CDatabase<'r>>>,
    schema_record_id: i32,
    object_tree: Rc<ObjectTreeEntry>,
    link_table: Rc<LinkTable>,
}

impl<'info, 'db> DataTable<'info, 'db> {
    /// create a new datatable wrapper
    pub fn new(
        data_table: cache::Table<'info, 'db, cache::DataTable>,
        object_tree: Rc<ObjectTreeEntry>,
        schema_record_id: i32,
        link_table: Rc<LinkTable>,
    ) -> Result<Self> {
        Ok(Self {
            //database: None,
            data_table,
            schema_record_id,
            object_tree,
            link_table,
        })
    }

    /*
       pub fn set_database(&mut self, database: Weak<CDatabase<'r>>) {
           self.database = Some(database);
       }

       pub(crate) fn data_table(&self) -> &CDataTable {
           &self.data_table
       }
    */
    fn find_type_record(
        &'db self,
        object_type: ObjectType,
    ) -> anyhow::Result<Option<DataTableRecord<'info, 'db>>> {
        let mut records = self.find_type_records(hashset! {object_type})?;
        Ok(records.remove(&object_type))
    }

    pub fn find_all_type_names(&self) -> Result<HashMap<i32, String>> {
        let mut type_records = HashMap::new();
        for dbrecord in self.data_table.children_of(self.schema_record_id) {
            let object_name2 = dbrecord.ds_object_name2()?.to_owned();

            type_records.insert(dbrecord.ds_record_id()?, object_name2);
        }
        log::info!("found all required type definitions");
        Ok(type_records)
    }

    pub fn find_type_records(
        &'db self,
        mut types: HashSet<ObjectType>,
    ) -> anyhow::Result<HashMap<ObjectType, DataTableRecord<'info, 'db>>> {
        let mut type_records = HashMap::new();
        let children = self.data_table.children_of(self.schema_record_id);
        if !children.count() > 0 {
            return Err(anyhow::anyhow!(Error::SchemaRecordHasNoChildren));
        }

        for dbrecord in self.data_table.children_of(self.schema_record_id) {
            let object_name2 = dbrecord.ds_object_name2()?.to_string();

            log::trace!("found a new type definition: '{}'", object_name2);

            if types.remove(&object_name2[..].try_into()?) {
                log::debug!("found requested type definition for '{object_name2}'");
                type_records.insert(ObjectType::try_from(&object_name2[..])?, dbrecord);
            }

            if types.is_empty() {
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
        for dbrecord in self.data_table.children_of(self.schema_record_id) {
            let object_name2 = dbrecord.ds_object_name2()?.to_owned();

            type_names.insert(object_name2);

            if type_names.is_empty() {
                break;
            }
        }
        let x = self
            .data_table
            .children_of(self.schema_record_id)
            .map(|dbrecord| dbrecord.ds_object_name2().unwrap().to_owned());
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

    pub fn show_tree(&self, max_depth: u8) -> Result<()> {
        let tree = ObjectTreeEntry::to_tree(&self.object_tree, max_depth);
        println!("{}", tree);
        Ok(())
    }

    pub fn show_entry(&self, entry_id: EntryId) -> Result<()> {
        let record = match entry_id {
            EntryId::Id(id) => self.data_table.find_p(RecordHasId(id)),
            EntryId::Rid(rid) => self.data_table.find_p(RecordHasRid(rid)),
        };

        match record {
            None => println!("no matching object found"),
            Some(entry) => {
                let mut table = term_table::Table::from(&entry);

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

    pub fn search_entries(&self, regex: &str) -> anyhow::Result<()> {
        let re = Regex::new(regex)?;
        let mut table_columns = vec![
            "DNT_col".to_owned(),
            "PDNT_col".to_owned(),
            "ATTm3".to_owned(),
            "ATTm589825".to_owned(),
            "ATTb590606".to_owned(),
        ];

        let mut records = Vec::new();

        for record in self.data_table.iter() {
            let matching_columns = record
                .all_attributes()
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
            let all_attributes = record.all_attributes();
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

    fn show_typed_objects<O: ntds::Object>(
        &self,
        options: &OutputOptions,
        object_type: ObjectType,
    ) -> anyhow::Result<()> {
        let type_record = self
            .find_type_record(object_type)?
            .unwrap_or_else(|| panic!("missing record for type '{object_type}'"));
        let type_record_id = type_record.ds_record_id()?;

        let mut csv_wtr = csv::Writer::from_writer(std::io::stdout());

        for record in self
            .data_table
            .iter()
            .filter(|dbrecord| dbrecord.ds_object_type_id().is_ok())
            .filter(|dbrecord| dbrecord.ds_object_type_id().unwrap() == type_record_id)
            .map(|dbrecord| O::new(dbrecord, options).unwrap())
        {
            match options.format() {
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

    pub fn show_timeline(&self, options: &OutputOptions) -> anyhow::Result<()> {
        let type_records = self.find_type_records(hashset! {
        ObjectType::Person,
        ObjectType::Computer})?;

        let all_type_records = self.find_all_type_names()?;

        let type_record_ids = if *options.show_all_objects() {
            None
        } else {
            Some(
                type_records
                    .iter()
                    .map(|(type_name, dbrecord)| {
                        (
                            dbrecord.ds_record_id().expect("unable to read record id"),
                            *type_name,
                        )
                    })
                    .collect::<HashMap<i32, ObjectType>>(),
            )
        };

        for bf_lines in self
            .data_table
            .iter()
            .filter(|dbrecord| dbrecord.ds_object_type_id_opt().unwrap().is_some())
            .filter_map(|dbrecord| {
                let current_type_id = dbrecord.ds_object_type_id().unwrap();

                // `type_record_ids` is None if `all_objects` is True
                if let Some(record_ids) = type_record_ids.as_ref() {
                    match record_ids.get(&current_type_id) {
                        Some(type_name) => {
                            if *type_name == ObjectType::Person {
                                match Person::new(dbrecord, options) {
                                    Ok(person) => Some(Vec::<Bodyfile3Line>::from(person)),
                                    Err(why) => {
                                        log::error!("unable to parse person: {why}");
                                        None
                                    }
                                }
                            } else if *type_name == ObjectType::Computer {
                                match Computer::new(dbrecord, options) {
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
}
