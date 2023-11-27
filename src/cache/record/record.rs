use std::hash::Hash;
use std::ops::Index;
use std::rc::Rc;

use dashmap::mapref::one::RefMut;
use dashmap::DashMap;
use libesedb::Value;

use crate::ntds::NtdsAttributeId;
use crate::ColumnInfoMapping;

pub trait EsedbRecord<'record>: Eq + PartialEq<Self> + Hash {}

impl<'record> EsedbRecord<'record> for Record<'record> {}
pub trait IsRecord {}
impl<R> IsRecord for R where for<'record> R: EsedbRecord<'record> {}

pub struct Record<'record> {
    table_id: &'static str,
    record_id: i32,
    values: DashMap<i32, Value>,
    count: i32,
    record: libesedb::Record<'record>,
    mapping: Rc<ColumnInfoMapping>,
}

impl Eq for Record<'_> {}

impl PartialEq<Self> for Record<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.record_id == other.record_id && self.table_id == other.table_id
    }
}

impl Hash for Record<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.table_id.hash(state);
        self.record_id.hash(state);
    }
}

impl<'record> Record<'record> {
    pub fn get_by_id(&self, attribute_id: NtdsAttributeId) -> Option<RefMut<'_, i32, Value>> {
        self.get_by_index(self.mapping.index(attribute_id).id())
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
        record: libesedb::Record<'record>,
        table_id: &'static str,
        record_id: i32,
        mapping: Rc<ColumnInfoMapping>,
    ) -> std::io::Result<Self> {
        Ok(Self {
            values: Default::default(),
            count: record.count_values()?,
            record,
            mapping,
            table_id,
            record_id
        })
    }
}
