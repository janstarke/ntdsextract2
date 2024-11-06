use crate::cache::{ColumnIndex, Value};
use crate::ntds::{Error, Result};

pub trait FromValue {
    /// does the same as [`from_value_opt`], but returns an Error instead of `None`, if no value was found
    fn from_value(value: &Value) -> Result<Self>
    where
        Self: Sized,
    {
        Self::from_value_opt(value)?.ok_or(Error::ValueIsMissing)
    }

    /// converts the value into the requested type, if possible
    fn from_value_opt(value: &Value) -> Result<Option<Self>>
    where
        Self: Sized;

    fn from_record_opt(
        record: &libesedb::Record,
        record_id: &ColumnIndex,
    ) -> anyhow::Result<Option<Self>>
    where
        Self: Sized,
    {
        if record.is_long(**record_id)? {
            let v = record.long(**record_id)?;
            let value = match v.variant() {
                libesedb::Value::Null(_) => Value::Null(()),
                libesedb::Value::Bool(_) => panic!("A Bool should not be stored as Long"),
                libesedb::Value::U8(_) => panic!("A U8 should not be stored as Long"),
                libesedb::Value::I16(_) => panic!("An I16 should not be stored as Long"),
                libesedb::Value::I32(_) => panic!("An I32 should not be stored as Long"),
                libesedb::Value::Currency(_) => panic!("A Currency should not be stored as Long"),
                libesedb::Value::F32(_) => panic!("A F32 should not be stored as Long"),
                libesedb::Value::F64(_) => panic!("A F64 should not be stored as Long"),
                libesedb::Value::DateTime(_) => panic!("A DateTime should not be stored as Long"),
                libesedb::Value::Binary(_) => Value::Binary(Box::new(v.vec()?)),
                libesedb::Value::Text(_) => Value::Text(Box::new(v.utf8()?)),
                libesedb::Value::LargeBinary(_) => Value::LargeBinary(Box::new(v.vec()?)),
                libesedb::Value::LargeText(_) => Value::LargeText(Box::new(v.utf8()?)),
                libesedb::Value::SuperLarge(_) => Value::SuperLarge(Box::new(v.vec()?)),
                libesedb::Value::U32(_) => panic!("A U32 should not be stored as Long"),
                libesedb::Value::I64(_) => panic!("An I64 should not be stored as Long"),
                libesedb::Value::Guid(_) => Value::Guid(Box::new(v.vec()?)),
                libesedb::Value::U16(_) => panic!("A U16 should not be stored as Long"),
                libesedb::Value::Long => panic!("A Long inside of a Long? This should not happen!"),
                libesedb::Value::Multi => {
                    panic!("A Multi inside of a Long? This should not happen!")
                }
            };
            Ok(Self::from_value_opt(&value)?)
        } else if record.is_multi(**record_id)? {
            let v = record.multi(**record_id)?;
            let mut values = Vec::new();
            for value in v.iter_values()? {
                values.push(Value::from(value?));
            }
            Ok(Self::from_value_opt(&Value::Multi(values))?)
        } else {
            Ok(Self::from_value_opt(&Value::from(
                record.value(**record_id)?,
            ))?)
        }
    }
}
