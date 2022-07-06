use libesedb::{self, Value};
use anyhow::{anyhow, Result};

use crate::ColumnInfoMapping;

macro_rules! define_i32_getter {
    ($fn_name: ident, $mapping_name: ident) => {
        
    pub fn $fn_name(&self, mapping: &ColumnInfoMapping) -> Result<i32> {
        let value = self.inner_record.value(mapping.$mapping_name.id)?;
        match value {
            Value::I32(val) => Ok(val),
            _ => Err(anyhow!("invalid value detected: {:?}", value))
        }
    }
    };
}


macro_rules! define_str_getter {
    ($fn_name: ident, $mapping_name: ident) => {
        
    pub fn $fn_name(&self, mapping: &ColumnInfoMapping) -> Result<String> {
        let value = self.inner_record.value(mapping.$mapping_name.id)?;
        match value {
            Value::Text(val) => Ok(val),
            Value::LargeText(val) => Ok(val),
            Value::Null => Ok("".to_string()),
            _ => Err(anyhow!("invalid value detected: {:?}", value))
        }
    }
    };
}

pub (crate) struct DbRecord<'a> {
    inner_record: libesedb::Record<'a>,
}

impl<'a> From<libesedb::Record<'a>> for DbRecord<'a> {
    fn from(inner: libesedb::Record<'a>) -> Self {
        Self {
            inner_record: inner
        }
    }
}

impl<'a> DbRecord<'a> {
    define_i32_getter!(ds_record_id_index, dsRecordIdIndex);
    define_i32_getter!(ds_parent_record_id_index, dsParentRecordIdIndex);

    pub fn ds_record_time_index(&self, mapping: &ColumnInfoMapping) -> Result<libesedb::Value, std::io::Error> {
        self.inner_record.value(mapping.dsRecordTimeIndex.id)
    }
    pub fn ds_ancestors_index(&self, mapping: &ColumnInfoMapping) -> Result<libesedb::Value, std::io::Error> {
        self.inner_record.value(mapping.dsAncestorsIndex.id)
    }
    define_i32_getter!(ds_object_type_id_index, dsObjectTypeIdIndex);

    define_str_getter!(ds_object_name_index, dsObjectNameIndex);
    define_str_getter!(ds_object_name2_index, dsObjectName2Index);

    define_str_getter!(ds_sidindex, ds_sidindex);
    define_str_getter!(ds_samaccount_name_index, ds_samaccount_name_index);
    define_str_getter!(ds_user_principal_name_index, ds_user_principal_name_index);
}