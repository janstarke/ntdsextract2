mod iter;
mod special_records;
mod table_type;

use anyhow::anyhow;
pub use iter::*;
pub use special_records::*;
pub use table_type::*;

use std::{marker::PhantomData, rc::Rc};

use crate::{
    cache, ntds::DataTableRecord, object_tree_entry::ObjectTreeEntry, EsedbInfo,
    RecordHasParent, RecordPredicate,
};

use super::RecordPointer;

pub struct Table<'info, 'db, T: TableType>
where
    'info: 'db,
{
    _table_id: &'static str,

    _table: &'info libesedb::Table<'db>,

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
        for (record_id, record) in table.iter_records().unwrap().enumerate() {
            records.push(
                cache::Record::try_from(
                    record.unwrap(),
                    table_id,
                    (record_id as i32).into(),
                    esedbinfo,
                    Rc::clone(&columns),
                )
                .unwrap(),
            );
        }
        log::info!(
            "successfully cached {} records of table '{table_id}'",
            records.len()
        );

        Ok(Self {
            _table: table,
            _table_id: table_id,
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
        parent_id: RecordPointer,
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

    pub fn find_by_id(&'db self, ptr: RecordPointer) -> Option<DataTableRecord<'info, 'db>> {
        if let Some(row) = ptr.esedb_row() {
            self.records
                .get(row.inner() as usize)
                .map(|r| DataTableRecord::new(r, *row))
        } else {
            self.iter().find(move |r| {
                ptr.ds_record_id()
                    == r.ds_record_id()
                        .expect("unable to read record id")
                        .ds_record_id()
            })
        }
    }

    pub fn get_special_records(&self, root: Rc<ObjectTreeEntry>) -> anyhow::Result<SpecialRecords> {
        log::info!("obtaining special record ids");

        // search downward until we find a `Configuration` entry
        let configuration_path = self
            .find_first_in_tree(&root, "Configuration")
            .ok_or(anyhow!("db has no `Configuration` entry"))?;

        let schema_subpath = self
            .find_child_by_name(&configuration_path[0], "Schema")
            .ok_or(anyhow!("db has no `Schema` entry"))?;

        let deleted_objects_subpath = self
            .find_child_by_name(&configuration_path[0], "Deleted Objects")
            .ok_or(anyhow!("db has no `Deleted Objects` entry"))?;

        Ok(SpecialRecords::new(
            schema_subpath,
            deleted_objects_subpath
        ))
    }

    pub fn path_to_str(&self, path: &Vec<Rc<ObjectTreeEntry>>) -> String {
        let v: Vec<_> = path.iter().map(|e| e.name().to_owned()).collect();
        v.join(",")
    }

    fn find_first_in_tree(
        &self,
        root: &Rc<ObjectTreeEntry>,
        name: &str,
    ) -> Option<Vec<Rc<ObjectTreeEntry>>> {
        if root.name() == name {
            Some(vec![Rc::clone(root)])
        } else {
            for child in root.children().borrow().iter() {
                if let Some(mut path) = self.find_first_in_tree(child, name) {
                    path.push(Rc::clone(root));
                    return Some(path);
                }
            }
            None
        }
    }

    fn find_child_by_name(
        &self,
        root: &Rc<ObjectTreeEntry>,
        name: &str,
    ) -> Option<Rc<ObjectTreeEntry>> {
        root.children()
            .borrow()
            .iter()
            .find(|e| e.name() == name)
            .map(|e| Rc::clone(e))
    }
}
