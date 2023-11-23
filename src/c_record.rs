use std::cell::{Ref, RefMut};
use std::collections::hash_map::Entry;
use std::ops::{Deref, Index};
use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};

use libesedb::{Record, Value};

use crate::ntds::NtdsAttributeId;
use crate::ColumnInfoMapping;

pub struct CRecord<'r> {
    values: RefCell<HashMap<i32, Option<Value>>>,
    count: i32,
    record: Record<'r>,
    mapping: Rc<ColumnInfoMapping>,
}

pub struct RecordValue<'c, 'r> {
    hashmap: RefMut<'c, HashMap<i32, Option<Value>>>,
    record: &'c Record<'r>,
    index: i32,
}

impl<'c, 'r> Deref for RecordValue<'c, 'r> {
    type Target = Option<Value>;

    fn deref(&self) -> &Self::Target {
        match self.hashmap.entry(self.index) {
            Entry::Occupied(e) => e.get(),
            Entry::Vacant(e) => match self.record.value(self.index) {
                Ok(v) => e.insert(Some(v)),
                Err(why) => panic!("invalid value for index '{}': {why}", self.index),
            },
        }
    }
}

impl<'r> CRecord<'r> {
    pub fn get_by_id(&self, attribute_id: NtdsAttributeId) -> &Option<Value> {
        static INVALID_COLUMN: Option<Value> = None;
        self.mapping
            .index(attribute_id)
            .map(|col_index| self.get_by_index(col_index))
            .unwrap_or(&INVALID_COLUMN)
    }

    pub fn get_by_index(&self, index: i32) -> &Option<Value> {
        self.value(index).deref()
    }

    fn value<'c>(&'c self, index: i32) -> RecordValue<'c, 'r> {
        RecordValue {
            hashmap: RefMut::map(self.values.borrow_mut(), |v| v),
            record: &self.record,
            index,
        }
    }

    pub fn try_from(record: Record<'r>, mapping: Rc<ColumnInfoMapping>) -> std::io::Result<Self> {
        Ok(Self {
            values: Default::default(),
            count: record.count_values()?,
            record,
            mapping,
        })
    }
}
