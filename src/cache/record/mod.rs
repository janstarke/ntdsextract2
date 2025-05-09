mod with_value;

pub use with_value::*;

use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Index;
use std::rc::Rc;

use getset::Getters;

use crate::cache::{ColumnIndex, Value};
use crate::esedb_mitigation::libesedb_count;
use crate::ntds::NtdsAttributeId;
use crate::EsedbInfo;

use super::{ColumnsOfTable, EsedbRowId};

#[derive(Getters)]
#[getset(get = "pub")]
pub struct Record<'info, 'db> {
    table_id: &'static str,
    row_id: EsedbRowId,

    #[getset(skip)]
    values: RefCell<HashMap<ColumnIndex, Option<Value>>>,

    count: i32,
    record: libesedb::Record<'db>,
    esedbinfo: &'info EsedbInfo<'db>,

    // this is needed for `::all_attributes`
    columns: Rc<ColumnsOfTable>,
}

impl Eq for Record<'_, '_> {}

impl PartialEq<Self> for Record<'_, '_> {
    fn eq(&self, other: &Self) -> bool {
        self.row_id == other.row_id && self.table_id == other.table_id
    }
}

impl Hash for Record<'_, '_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.table_id.hash(state);
        self.row_id.hash(state);
    }
}

impl<'info, 'db> WithValue<NtdsAttributeId> for Record<'info, 'db> {
    fn with_value<T>(
        &self,
        attribute_id: NtdsAttributeId,
        function: impl FnMut(Option<&Value>) -> crate::ntds::Result<T>,
    ) -> crate::ntds::Result<T> {
        let column_id = *self.esedbinfo().mapping().index(attribute_id).id();
        self.with_value(column_id, function)
    }
}

impl<'info, 'db> WithValue<ColumnIndex> for Record<'info, 'db> {
    fn with_value<T>(
        &self,
        index: ColumnIndex,
        mut function: impl FnMut(Option<&Value>) -> crate::ntds::Result<T>,
    ) -> crate::ntds::Result<T> {
        match self.values.borrow_mut().entry(index) {
            Entry::Occupied(e) => function(e.get().as_ref()),
            Entry::Vacant(e) => match self.record.value(*index) {
                Ok(v) => function(
                    e.insert(match v {
                        libesedb::Value::Null(()) => None,
                        libesedb::Value::Long => {
                            let x = self.record.long(*index)?;
                            Some(Value::Long(Box::new(x.vec()?)))
                        }
                        libesedb::Value::Multi => {
                            let v = self.record.multi(*index)?.variant();
                            Some(v.into())
                        }
                        v => Some(v.into()),
                    })
                    .as_ref(),
                ),
                Err(why) => Err(why.into()),
            },
        }
    }
}

impl<'info, 'db> Record<'info, 'db> {
    pub fn try_from(
        record: libesedb::Record<'db>,
        table_id: &'static str,
        row_id: EsedbRowId,
        esedbinfo: &'info EsedbInfo<'db>,
        columns: Rc<ColumnsOfTable>,
    ) -> std::io::Result<Self> {
        Ok(Self {
            values: Default::default(),
            count: libesedb_count(|| record.count_values())?,
            record,
            esedbinfo,
            table_id,
            row_id,
            columns,
        })
    }
}
