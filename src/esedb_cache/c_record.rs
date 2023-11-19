use std::{cell::RefCell, collections::HashMap};
use std::collections::hash_map::Entry;

use libesedb::{Record, Value};

use crate::IsRecord;

pub struct CRecord<'r> {
    values: RefCell<HashMap<i32, Value>>,
    count: i32,
    record: Record<'r>,
}

impl<'a> TryFrom<Record<'a>> for CRecord<'_> {
    type Error = std::io::Error;

    fn try_from(record: Record<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            values: Default::default(),
            count: record.count_values()?,
            record,
        })
        /*
        let mut values = Vec::new();
        for value in record.iter_values()? {
            let value = match value {
                Ok(v) => v,
                Err(why) => {
                    log::trace!("unable to read value: {why}, replacing it with Null");
                    Value::Null(())
                }
            };
            values.push(value);
        }
        Ok(Self {
            values
        })
         */
    }
}

impl IsRecord for CRecord<'_> {
    fn count_values(&self) -> i32 {
        self.count
    }

    fn with_value_mut<F>(&self, index: i32, mut action: F) where F: FnMut(&Value) {
        match self.values.borrow().entry(index) {
            Entry::Occupied(e) => action(e.get()),
            Entry::Vacant(e) => {
                match self.record.value(index) {
                    Ok(v) => {
                        self.values.borrow_mut().insert(index, v);
                        action(&v);
                    }
                    Err(why) => {
                        log::error!("invalid no value for index '{index}': {why}");
                    }
                }
            },
        }
    }
}
