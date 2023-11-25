use std::ops::Index;
use std::rc::Rc;

use dashmap::DashMap;
use dashmap::mapref::one::RefMut;
use libesedb::{Record, Value};

use crate::ntds::NtdsAttributeId;
use crate::ColumnInfoMapping;

pub trait EsedbRecord<'record> {

}

impl<'record> EsedbRecord<'record> for CRecord<'record> {

}
pub trait IsRecord {}
impl<R> IsRecord for R where for<'record> R: EsedbRecord<'record> {}

pub struct CRecord<'record> {
    values: DashMap<i32, Value>,
    count: i32,
    record: Record<'record>,
    mapping: Rc<ColumnInfoMapping>,
}

impl<'record> CRecord<'record> {
    pub fn get_by_id(&self, attribute_id: NtdsAttributeId) -> Option<RefMut<'_, i32, Value>> {
        self.get_by_index(self.mapping.index(attribute_id).id())
    }

    pub fn get_by_index(&self, index: i32) -> Option<RefMut<'_, i32, Value>> {
        self.value(index)
    }

    fn value(&self, index: i32) -> Option<RefMut<'_, i32, Value>> {
        self
            .values
            .entry(index)
            .or_try_insert_with(|| self.record.value(index))
            .map_err(|_why| log::error!("invalid index '{index}'"))
            .ok()
    }

    pub fn try_from(record: Record<'record>, mapping: Rc<ColumnInfoMapping>) -> std::io::Result<Self> {
        Ok(Self {
            values: Default::default(),
            count: record.count_values()?,
            record,
            mapping,
        })
    }
}
