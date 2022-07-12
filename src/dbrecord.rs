use libesedb::{self, Value};
use anyhow::{anyhow, Result};
use std::{io::Cursor, collections::HashMap};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use chrono::{DateTime, Utc, TimeZone, Duration, NaiveDate};

use crate::{column_info_mapping::ColumnInfoMapping, user_account_control::UserAccountControl};

macro_rules! define_i32_getter {
    ($fn_name: ident, $mapping_name: ident) => {
        
    pub fn $fn_name(&self, mapping: &ColumnInfoMapping) -> Result<Option<i32>> {
        let value = self.inner_record.value(mapping.$mapping_name.id())?;
        match value {
            Value::I32(val) => Ok(Some(val)),
            Value::Null => Ok(None),
            _ => Err(anyhow!("invalid value detected: {:?} in field {}", value, stringify!($fn_name)))
        }
    }
    };
}

macro_rules! define_flags_getter {
    ($fn_name: ident, $mapping_name: ident, $flags_type: ty) => {
        
    pub fn $fn_name(&self, mapping: &ColumnInfoMapping) -> Result<Option<$flags_type>> {
        let value = self.inner_record.value(mapping.$mapping_name.id())?;
        match value {
            Value::I32(val) =>
                Ok(Some(<$flags_type>::from_bits_truncate(u32::from_ne_bytes(val.to_ne_bytes())))),
            Value::Null => Ok(None),
            _ => Err(anyhow!("invalid value detected: {:?} in field {}", value, stringify!($fn_name)))
        }
    }
    };
}

macro_rules! define_str_getter {
    ($fn_name: ident, $mapping_name: ident) => {
        
    pub fn $fn_name(&self, mapping: &ColumnInfoMapping) -> Result<Option<String>> {
        let value = self.inner_record.value(mapping.$mapping_name.id())?;
        match value {
            Value::Text(val) => Ok(Some(val)),
            Value::LargeText(val) => Ok(Some(val)),
            Value::Null => Ok(None),
            _ => Err(anyhow!("invalid value detected: {:?} in field {}", value, stringify!($fn_name)))
        }
    }
    };
}

macro_rules! define_bin_getter {
    ($fn_name: ident, $mapping_name: ident) => {
        
    pub fn $fn_name(&self, mapping: &ColumnInfoMapping) -> Result<Option<String>> {
        let value = self.inner_record.value(mapping.$mapping_name.id())?;
        match value {
            Value::Binary(val) | Value::LargeBinary(val) => {
                Ok(Some(hex::encode(val)))
            }
            Value::Null => Ok(None),
            _ => Err(anyhow!("invalid value detected: {:?} in field {}", value, stringify!($fn_name)))
        }
    }
    };
}


/// https://devblogs.microsoft.com/oldnewthing/20040315-00/?p=40253
macro_rules! define_sid_getter {
    ($fn_name: ident, $mapping_name: ident) => {
        
    pub fn $fn_name(&self, mapping: &ColumnInfoMapping) -> Result<Option<String>> {
        let value = self.inner_record.value(mapping.$mapping_name.id())?;
        match value {
            Value::Binary(val) | Value::LargeBinary(val) => {
                //log::debug!("val: {:?}", val);
                let mut rdr = Cursor::new(val);
                let revision = rdr.read_u8()?;
                let number_of_dashes = rdr.read_u8()?;
                let authority = rdr.read_u48::<BigEndian>()?;

                //log::debug!("authority: {:012x}", authority);

                let mut numbers = vec![];
                for _i in 0..number_of_dashes-1 {
                    numbers.push(rdr.read_u32::<LittleEndian>()?);
                }
                numbers.push(rdr.read_u32::<BigEndian>()?);

                let numbers = numbers
                    .into_iter()
                    .map(|n| format!("{n}")).collect::<Vec<String>>().join("-");

                Ok(Some(format!("S-{revision}-{authority}-{numbers}")))
            }
            Value::Null => Ok(None),
            _ => Err(anyhow!("invalid value detected: {:?} in field {}", value, stringify!($fn_name)))
        }
    }
    };
}

macro_rules! define_datetime_getter {
    ($fn_name: ident, $mapping_name: ident) => {
        
    pub fn $fn_name(&self, mapping: &ColumnInfoMapping) -> Result<Option<DateTime<Utc>>> {
        let value = self.inner_record.value(mapping.$mapping_name.id())?;
        match value {
            Value::Currency(val) => Ok(Some(currency_to_datetime(val))),
            Value::Null => Ok(None),
            _ => Err(anyhow!("invalid value detected: {:?} in field {}", value, stringify!($fn_name)))
        }
    }
    };
}

fn currency_to_datetime(val: i64) -> DateTime<Utc> {
    let dt_base = DateTime::<Utc>::from_utc(NaiveDate::from_ymd(1601, 1, 1).and_hms(0, 0, 0), Utc);
    let duration = Duration::microseconds(val / 10);
    dt_base + duration
}

pub (crate) trait FromDbRecord where Self: Sized {
    fn from(dbrecord: DbRecord, mapping: &ColumnInfoMapping) -> Result<Self>;
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

    define_datetime_getter!(ds_record_time_index, ds_record_time_index);
    define_i32_getter!(ds_ancestors_index, ds_ancestors_index);
    define_i32_getter!(ds_object_type_id_index, dsObjectTypeIdIndex);

    define_str_getter!(ds_object_name_index, dsObjectNameIndex);
    define_str_getter!(ds_object_name2_index, dsObjectName2Index);

    define_datetime_getter!(ds_when_created_index, ds_when_created_index);
    define_datetime_getter!(ds_when_changed_index, ds_when_changed_index);

    define_sid_getter!(ds_sidindex, ds_sidindex);
    define_str_getter!(ds_samaccount_name_index, ds_samaccount_name_index);
    define_str_getter!(ds_user_principal_name_index, ds_user_principal_name_index);
    define_i32_getter!(ds_samaccount_type_index, ds_samaccount_type_index);
    define_flags_getter!(ds_user_account_control_index, ds_user_account_control_index, UserAccountControl);
    define_datetime_getter!(ds_last_logon_index, ds_last_logon_index);
    define_datetime_getter!(ds_last_logon_time_stamp_index, ds_last_logon_time_stamp_index);
    define_datetime_getter!(ds_account_expires_index, ds_account_expires_index);
    define_datetime_getter!(ds_password_last_set_index, ds_password_last_set_index);
    define_datetime_getter!(ds_bad_pwd_time_index, ds_bad_pwd_time_index);
    define_i32_getter!(ds_logon_count_index, ds_logon_count_index);
    define_i32_getter!(ds_bad_pwd_count_index, ds_bad_pwd_count_index);
    define_i32_getter!(ds_primary_group_id_index, ds_primary_group_id_index);
    define_bin_getter!(ds_nthash_index, ds_nthash_index);
    define_bin_getter!(ds_lmhash_index, ds_lmhash_index);
    define_bin_getter!(ds_nthash_history_index, ds_nthash_history_index);
    define_bin_getter!(ds_lmhash_history_index, ds_lmhash_history_index);
    define_str_getter!(ds_unix_password_index, ds_unix_password_index);
    define_bin_getter!(ds_aduser_objects_index, ds_aduser_objects_index);
    define_bin_getter!(ds_supplemental_credentials_index, ds_supplemental_credentials_index);
    define_str_getter!(ds_att_comment, ds_att_comment);


    define_str_getter!(dnshost_name, dnshost_name);
    define_str_getter!(osname, osname);
    define_str_getter!(osversion, osversion);

    pub fn all_attributes(&self, mapping: &ColumnInfoMapping) -> HashMap<String, String> {
        let mut attribs = HashMap::new();
        for index in 0..self.inner_record.count_values().unwrap() {
            if let Ok(value) = self.inner_record.value(index) {
                if ! matches!(value, Value::Null) {
                    if let Some(column_name) = mapping.name_of_column(&index) {
                        let str_value = match value {
                            Value::Null => panic!("unreachable code executed"),
                            Value::Bool(v) => format!("{v}"),
                            Value::U8(v) => format!("{v}"),
                            Value::I16(v) => format!("{v}"),
                            Value::I32(v) => format!("{v}"),
                            Value::Currency(v) => format!("{v}"),
                            Value::F32(v) => format!("{v}"),
                            Value::F64(v) => format!("{v}"),
                            Value::DateTime(v) => format!("{v}"),
                            Value::Binary(v) => format!("{}", hex::encode(&v)),
                            Value::Text(v) => format!("{v}"),
                            Value::LargeBinary(v) => format!("{}", hex::encode(&v)),
                            Value::LargeText(v) => v,
                            Value::SuperLarge(v) => format!("{}", hex::encode(&v)),
                            Value::U32(v) => format!("{v}"),
                            Value::I64(v) => format!("{v}"),
                            Value::Guid(v) => format!("{}", hex::encode(&v)),
                            Value::U16(v) => format!("{v}"),
                        };
                        attribs.insert(column_name.to_owned(), str_value);
                    }
                }
            }
        }
        attribs
    }
}