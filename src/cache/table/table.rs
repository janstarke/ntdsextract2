use std::{marker::PhantomData, rc::Rc};

use crate::{
    cache, ntds::DataTableRecord, EsedbInfo, RecordHasAttRdn, RecordHasId, RecordHasParent,
    RecordPredicate,
};

use super::{DataTable, Iter, LinkTable, TableType};

pub struct Table<'info, 'db, T: TableType>
where
    'info: 'db,
{
    table_id: &'static str,

    table: &'info libesedb::Table<'db>,

    columns: Rc<Vec<cache::Column>>,

    records: Vec<cache::Record<'info, 'db>>,

    _marker: PhantomData<T>,
}

impl<'info, 'db, T: TableType> Table<'info, 'db, T>
where
    'info: 'db,
{
    pub fn try_from(
        table: &'info libesedb::Table<'db>,
        table_id: &'static str,
        esedbinfo: &'info EsedbInfo<'db>,
    ) -> std::io::Result<Self> {
        let mut columns = Vec::new();

        for column in table.iter_columns()? {
            columns.push(cache::Column::try_from(column?)?);
        }
        let columns = Rc::new(columns);

        let mut records = Vec::new();
        for (mut record_id, record) in table.iter_records().unwrap().enumerate() {
            records.push(
                cache::Record::try_from(
                    record.unwrap(),
                    table_id,
                    record_id as i32,
                    esedbinfo,
                    Rc::clone(&columns),
                )
                .unwrap(),
            );
            record_id += 1;
        }

        Ok(Self {
            table,
            table_id,
            records,
            columns,
            _marker: PhantomData,
        })
    }

    pub fn count_columns(&self) -> i32 {
        self.columns.len().try_into().unwrap()
    }

    pub fn column(&self, pos: i32) -> Option<&cache::Column> {
        self.columns.get(usize::try_from(pos).unwrap())
    }
}

impl<'info, 'db> Table<'info, 'db, LinkTable> {
    pub fn iter(&'db self) -> impl Iterator<Item = &'info cache::Record<'info, 'db>> {
        self.records.iter()
    }
}
impl<'info, 'db> Table<'info, 'db, DataTable> {
    pub fn iter(&'db self) -> Iter<'info, 'db> {
        self.records.iter().into()
    }

    pub(crate) fn children_of(
        &'db self,
        parent_id: i32,
    ) -> impl Iterator<Item = DataTableRecord<'info, 'db>> {
        let my_filter = RecordHasParent(parent_id);
        self.iter().filter(move |r| my_filter.matches(r))
    }

    pub fn filter<C>(&'db self, predicate: C) -> impl Iterator<Item = DataTableRecord<'info, 'db>>
    where
        C: Fn(&DataTableRecord<'info, 'db>) -> bool,
    {
        self.iter().filter(move |r| predicate(r))
    }

    pub fn find<C>(&'db self, predicate: C) -> Option<DataTableRecord<'info, 'db>>
    where
        C: Fn(&DataTableRecord<'info, 'db>) -> bool,
    {
        self.iter().find(move |r| predicate(r))
    }

    pub fn filter_p<P>(&'db self, predicate: P) -> impl Iterator<Item = DataTableRecord<'info, 'db>>
    where
        P: RecordPredicate<'info, 'db>,
    {
        self.iter().filter(move |r| predicate.matches(r))
    }

    pub fn find_p<P>(&'db self, predicate: P) -> Option<DataTableRecord<'info, 'db>>
    where
        P: RecordPredicate<'info, 'db>,
    {
        self.iter().find(move |r| predicate.matches(r))
    }

    /// returns the record id of the record which contains the Schema object
    /// (which is identified by its name "Schema" in the object_name2 attribute)
    pub fn get_schema_record_id(&self) -> crate::ntds::Result<i32> {
        log::info!("obtaining schema record id");

        for record in self
            .filter_p(RecordHasAttRdn("Schema"))
            .map(DataTableRecord::from)
        {
            if let Some(schema_parent_id) = record.ds_parent_record_id_opt()? {
                if let Some(schema_parent) = self.find_p(RecordHasId(schema_parent_id)) {
                    if let Some(parent_name) = schema_parent.att_object_name2_opt()? {
                        if parent_name == "Configuration" {
                            log::info!("found record id to be {}", record.ds_record_id()?);
                            return Ok(record.ds_record_id()?);
                        }
                    }
                }
            }
        }
        Err(crate::ntds::Error::MissingSchemaRecord)
    }
}
