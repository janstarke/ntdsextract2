use std::hash::Hash;
use std::ops::Index;
use std::rc::Rc;

use dashmap::mapref::one::RefMut;
use dashmap::DashMap;
use getset::Getters;
use libesedb::Value;

use crate::cache;
use crate::ntds::NtdsAttributeId;
use crate::EsedbInfo;

#[derive(Getters)]
#[getset(get = "pub")]
pub struct Record<'info, 'db> {
    table_id: &'static str,
    record_id: i32,
    values: DashMap<i32, Value>,
    count: i32,
    record: libesedb::Record<'db>,
    esedbinfo: &'info EsedbInfo<'db>,
    columns: Rc<Vec<cache::Column>>
}

impl Eq for Record<'_, '_> {}

impl PartialEq<Self> for Record<'_, '_> {
    fn eq(&self, other: &Self) -> bool {
        self.record_id == other.record_id && self.table_id == other.table_id
    }
}

impl Hash for Record<'_, '_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.table_id.hash(state);
        self.record_id.hash(state);
    }
}

impl<'info, 'db> Record<'info, 'db> {
    pub fn get_by_id(&self, attribute_id: NtdsAttributeId) -> Option<RefMut<'_, i32, Value>> {
        self.get_by_index(self.esedbinfo().mapping().index(attribute_id).id())
    }

    pub fn get_by_index(&self, index: i32) -> Option<RefMut<'_, i32, Value>> {
        self.value(index)
    }

    fn value(&self, index: i32) -> Option<RefMut<'_, i32, Value>> {
        self.values
            .entry(index)
            .or_try_insert_with(|| self.record.value(index))
            .map_err(|_why| log::error!("invalid index '{index}'"))
            .ok()
    }

    pub fn try_from(
        record: libesedb::Record<'db>,
        table_id: &'static str,
        record_id: i32,
        esedbinfo: &'info EsedbInfo<'db>,
        columns: Rc<Vec<cache::Column>>
    ) -> std::io::Result<Self> {
        Ok(Self {
            values: Default::default(),
            count: record.count_values()?,
            record,
            esedbinfo,
            table_id,
            record_id,
            columns
        })
    }
}
