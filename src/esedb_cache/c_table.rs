use anyhow::bail;
use libesedb::{Record, Table};

use super::{CColumn, CRecord};
use crate::esedb_utils::FromValue;
use crate::{
    column_info_mapping::{ColumnInfoMapping, DbRecord},
    win32_types::Sid,
};

pub(crate) type CDataTable = CTable<DbRecord>;
pub(crate) type CLinkTable = CTable<CRecord>;

pub struct CTable<R>
where
    for<'r> R: TryFrom<Record<'r>, Error = std::io::Error>,
{
    records: Vec<R>,
    columns: Vec<CColumn>,
}

impl<'a, R> TryFrom<Table<'a>> for CTable<R>
where
    for<'r> R: TryFrom<Record<'r>, Error = std::io::Error>,
{
    type Error = std::io::Error;

    fn try_from(table: Table<'a>) -> Result<Self, Self::Error> {
        let mut records = Vec::new();
        let mut columns = Vec::new();
        for record in table.iter_records()? {
            records.push(R::try_from(record?)?);
        }
        for column in table.iter_columns()? {
            columns.push(CColumn::try_from(column?)?);
        }

        Ok(Self { records, columns })
    }
}

impl<R> CTable<R>
where
    for<'r> R: TryFrom<Record<'r>, Error = std::io::Error>,
{
    pub fn count_columns(&self) -> i32 {
        self.columns.len().try_into().unwrap()
    }

    pub fn iter_records(&self) -> impl Iterator<Item = &R> {
        self.records.iter()
    }

    pub fn column(&self, pos: i32) -> Option<&CColumn> {
        self.columns.get(usize::try_from(pos).unwrap())
    }

    pub fn filter_records_from<'a, 'b, P>(&'a self, predicate: P) -> impl Iterator<Item = &'a R>
    where
        P: Fn(&'a R) -> bool + 'b,
        'b: 'a,
    {
        self.iter_records().filter(move |r| predicate(*r)).map(|r| {
            log::trace!("found one object");
            r
        })
    }

    pub fn find_record_from<'a, 'b, P>(&'a self, predicate: P) -> Option<&'a R>
    where
        P: Fn(&'a R) -> bool + 'b,
        'b: 'a,
    {
        self.iter_records().find(move |r| predicate(*r))
    }
}

impl CTable<DbRecord> {
    pub fn find_by_id<'a, 'b>(
        &'a self,
        mapping: &'b ColumnInfoMapping,
        index: i32,
    ) -> Option<&'a DbRecord>
    where
        'b: 'a,
    {
        self.find_record_from(move |d| {
            d.ds_record_id(mapping)
                .expect("unable to read object record id")
                .expect("object has no record id")
                == index
        })
    }

    pub fn find_by_rid<'a, 'b>(
        &'a self,
        mapping: &'b ColumnInfoMapping,
        object_rid: u32,
    ) -> Option<&'a DbRecord>
    where
        'b: 'a,
    {
        self.find_record_from(move |d| {
            d.value_of_ds_sid(mapping)
                .and_then(|sid| {
                    Sid::from_value_opt(sid, "ATTj589922")
                        .ok()
                        .flatten()
                        .map(|sid| sid.get_rid() == &object_rid)
                })
                .unwrap_or(false)
        })
    }

    pub(crate) fn find_children_of<'a, 'b>(
        &'a self,
        mapping: &'b ColumnInfoMapping,
        parent_id: i32,
    ) -> impl Iterator<Item = &'a DbRecord>
    where
        'b: 'a,
    {
        log::debug!("searching for children of record '{}'", parent_id);

        self.filter_records_from(move |dbrecord| {
            dbrecord.ds_parent_record_id(mapping).unwrap().unwrap() == parent_id
        })
    }

    pub fn find_children_of_int<'a, 'b>(
        &'a self,
        mapping: &'b ColumnInfoMapping,
        parent_id: i32,
    ) -> impl Iterator<Item = &'a DbRecord>
    where
        'b: 'a,
    {
        self.find_children_of(mapping, parent_id)
    }

    /// returns the record id of the record which contains the Schema object
    /// (which is identified by its name "Schema" in the object_name2 attribute)
    pub fn get_schema_record_id(&self, mapping: &ColumnInfoMapping) -> anyhow::Result<i32> {
        log::info!("obtaining schema record id");

        for record in self.filter_records_from(|dbrecord| {
            "Schema"
                == dbrecord
                    .ds_object_name2(mapping)
                    .expect("unable to read object_name2 attribute")
                    .expect("missing object_name2 attribute")
        }) {
            if let Some(schema_parent_id) = record.ds_parent_record_id(mapping)? {
                if let Some(schema_parent) = self.find_by_id(mapping, schema_parent_id) {
                    if let Some(parent_name) = schema_parent.ds_object_name2(mapping)? {
                        if parent_name == "Configuration" {
                            log::info!("found record id to be {}", record.ds_record_id(mapping)?.unwrap());
                            return Ok(record
                                .ds_record_id(mapping)?
                                .expect("Schema record has no record ID"));
                        }
                    }
                }
            }
        }

        bail!("no schema record found");
    }
}
