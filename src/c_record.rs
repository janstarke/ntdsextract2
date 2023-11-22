use std::collections::hash_map::Entry;
use std::ops::Index;
use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};

use libesedb::{Record, Value};

use crate::ntds::NtdsAttributeId;
use crate::ColumnInfoMapping;

pub struct CRecord<'r> {
    values: RefCell<HashMap<i32, Value>>,
    count: i32,
    record: Record<'r>,
    mapping: Rc<ColumnInfoMapping>,
}

impl<'r> CRecord<'r> {
    pub fn get(&self, attribute_id: NtdsAttributeId) -> Option<&Value> {
        self.mapping.index(attribute_id).and_then(|col_index| self.get_value_in_column(col_index))
    }

    pub fn get_value_in_column(&self, index: i32) -> Option<&Value> {
        match self.values.borrow_mut().entry(index) {
            Entry::Occupied(e) => Some(e.get()),
            Entry::Vacant(e) => match self.record.value(index) {
                Ok(v) => Some(e.insert(v)),
                Err(why) => {
                    log::error!("invalid value for index '{index}': {why}");
                    None
                }
            },
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
