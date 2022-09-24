use std::{collections::HashMap, io::Cursor};
use bodyfile::Bodyfile3Line;
use byteorder::{BigEndian, ReadBytesExt, LittleEndian};
use chrono::{DateTime, Utc, NaiveDate, Duration};
use libesedb::{Table, Value};
use crate::{column_information::ColumnInformation, win32_types::*};
use anyhow::{Result, anyhow}; 
use num_traits::FromPrimitive;
use paste::paste;

fn value_to_i32(value: Value, attrib_name: &str) -> Result<Option<i32>> {
    match value {
        Value::I32(val) => Ok(Some(val)),
        Value::Null => Ok(None),
        _ => Err(anyhow!("invalid value detected: {:?} in field {}", value, attrib_name))
    }
}

fn value_to_str(value: Value, attrib_name: &str) -> Result<Option<String>> {
    match value {
        Value::Text(val) => Ok(Some(val)),
        Value::LargeText(val) => Ok(Some(val)),
        Value::Null => Ok(None),
        _ => Err(anyhow!("invalid value detected: {:?} in field {}", value, attrib_name))
    }
}

type UtcDatetime = DateTime<Utc>;
fn value_to_datetime(value: Value, attrib_name: &str) -> Result<Option<UtcDatetime>> {
    match value {
        Value::Currency(val) => Ok(Some(currency_to_datetime(val))),
        Value::Null => Ok(None),
        _ => Err(anyhow!("invalid value detected: {:?} in field {}", value, attrib_name))
    }
}

fn currency_to_datetime(val: i64) -> DateTime<Utc> {
    let dt_base = DateTime::<Utc>::from_utc(NaiveDate::from_ymd(1601, 1, 1).and_hms(0, 0, 0), Utc);
    let duration = Duration::microseconds(val / 10);
    dt_base + duration
}

fn value_to_bin(value: Value, attrib_name: &str) -> Result<Option<String>> {
    match value {
        Value::Binary(val) | Value::LargeBinary(val) => {
            Ok(Some(hex::encode(val)))
        }
        Value::Null => Ok(None),
        _ => Err(anyhow!("invalid value detected: {:?} in field {}", value, attrib_name))
    }
}

/// https://devblogs.microsoft.com/oldnewthing/20040315-00/?p=40253
fn value_to_sid(value: Value, attrib_name: &str) -> Result<Option<String>> {
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
        _ => Err(anyhow!("invalid value detected: {:?} in field {}", value, attrib_name))
    }
}

fn value_to_uac_flags(value: Value, attrib_name: &str) -> Result<Option<UserAccountControl>> {
    match value {
        Value::I32(val) =>
            Ok(Some(<UserAccountControl>::from_bits_truncate(u32::from_ne_bytes(val.to_ne_bytes())))),
        Value::Null => Ok(None),
        _ => Err(anyhow!("invalid value detected: {:?} in field {}", value, attrib_name))
    }
}

fn value_to_sam_account_type(value: Value, attrib_name: &str) -> Result<Option<SamAccountType>> {
    match value {
        Value::I32(val) =>
            Ok(FromPrimitive::from_u32(u32::from_ne_bytes(val.to_ne_bytes()))),
        Value::Null => Ok(None),
        _ => Err(anyhow!("invalid value detected: {:?} in field {}", value, attrib_name))
    }
}

macro_rules! define_getter_int {
    ($field: ident, $fn_name: ident, $res_type: ident) => {
        #[allow(dead_code)]
        pub fn $field(&self, mapping: &ColumnInfoMapping) -> Result<Option<$res_type>> {
            $fn_name(
                self.inner_record.value(mapping.$field.id())?,
                stringify!($field)
            )
        }

        paste! {
            #[allow(dead_code)]
            pub fn [<has_valid_ $field>](&self, mapping: &ColumnInfoMapping) -> bool {
                match self.inner_record.value(mapping.$field.id()) {
                    Ok(value) => match $fn_name(value, stringify!($field)) {
                        Ok(Some(_)) => true,
                        _ => false
                    }
                    _ => false
                }
            }
        }
    };
}

macro_rules! define_getter {
    ($field: ident as i32) => { define_getter_int!($field, value_to_i32, i32); };
    ($field: ident as str) => { define_getter_int!($field, value_to_str, String); };
    ($field: ident as sid) => { define_getter_int!($field, value_to_sid, String); };
    ($field: ident as binary) => { define_getter_int!($field, value_to_bin, String); };
    ($field: ident as datetime) => { define_getter_int!($field, value_to_datetime, UtcDatetime); };
    ($field: ident as uac_flags) => { define_getter_int!($field, value_to_uac_flags, UserAccountControl); };
    ($field: ident as sam_account_type) => { define_getter_int!($field, value_to_sam_account_type, SamAccountType); };
}


macro_rules! column_mapping {
    (
        $StructName:ident { $($manual_fields:tt)* },
        $RecordStructName:ident,
        $($field:ident as $type: ident from $internal_id: expr),+ $(,)?
    ) => {
            #[allow(dead_code)]
            pub(crate) struct $StructName {
                $(                    
                    pub (crate) $field: ColumnInformation,
                )+
                $($manual_fields)*
            }

            impl $StructName {
                pub fn from(data_table: &Table) -> Result<Self> {
                    let mut temporary_mapping = HashMap::new();
                    let mut column_names = HashMap::new();
                    for index in 0..data_table.count_columns()? {
                        let column_res = data_table.column(index)?;
                        let col_info = ColumnInformation::new(
                            index,
                            // column_res.name()?,
                            // column_res.variant()?
                        );
                        column_names.insert(index, column_res.name()?);
                        temporary_mapping.insert(column_res.name()?, col_info);
                    }
            
                    let mapping = Self {
                        column_names,
                        $(
                            $field: temporary_mapping.remove($internal_id).expect(&format!("missing column '{}'", $internal_id)),
                        )+
                    };
                    Ok(mapping)
                }
                pub fn name_of_column(&self, index: &i32) -> Option<&String> {
                    self.column_names.get(index)
                }
            }

            pub(crate) struct $RecordStructName<'a> {
                inner_record: libesedb::Record<'a>,
            }

            impl<'a> $RecordStructName<'a> {
                $(
                    define_getter!($field as $type);
                )+
                pub fn all_attributes(&self, mapping: &$StructName) -> HashMap<String, String> {
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
                                        Value::Binary(v) => hex::encode(&v).to_string(),
                                        Value::Text(v) => v.to_string(),
                                        Value::LargeBinary(v) => hex::encode(&v).to_string(),
                                        Value::LargeText(v) => v,
                                        Value::SuperLarge(v) => hex::encode(&v).to_string(),
                                        Value::U32(v) => format!("{v}"),
                                        Value::I64(v) => format!("{v}"),
                                        Value::Guid(v) => hex::encode(&v).to_string(),
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

            pub (crate) trait FromDbRecord where Self: Sized {
                fn from(dbrecord: $RecordStructName, mapping: &$StructName) -> Result<Self>;
            }

            impl<'a> From<libesedb::Record<'a>> for $RecordStructName<'a> {
                fn from(inner: libesedb::Record<'a>) -> Self {
                    Self {
                        inner_record: inner
                    }
                }
            }
        }
    }

column_mapping! (
    ColumnInfoMapping {
        column_names: HashMap<i32, String>,
    },
    DbRecord,
    ds_record_id as i32 from "DNT_col",
    ds_parent_record_id as i32 from "PDNT_col",
    ds_record_time as datetime from "time_col",
    ds_ancestors as i32 from "Ancestors_col",
    ds_object_type_id as i32 from "ATTb590606",
    ds_object_name as str from "ATTm3",
    ds_object_name2 as str from "ATTm589825",
    // DS_OBJECT_GUID as i32 from "ATTk589826",
    ds_when_created as datetime from "ATTl131074",
    ds_when_changed as datetime from "ATTl131075",
    // DS_USNCREATED as datetime from "ATTq131091",
    // DS_USNCHANGED as datetime from "ATTq131192",
    // DS_OBJECT_COL as i32 from "OBJ_col",
    // DS_IS_DELETED as i32 from "ATTi131120",
    
    // DS_ORIG_CONTAINER_ID as i32 from "ATTb590605",
    
    ds_sid as sid from "ATTr589970",
    ds_sam_account_name as str from "ATTm590045",
    ds_user_principal_name as str from "ATTm590480",
    ds_sam_account_type as sam_account_type from "ATTj590126",
    ds_user_account_control as uac_flags from "ATTj589832",
    ds_last_logon as datetime from "ATTq589876",
    ds_last_logon_time_stamp as datetime from "ATTq591520",
    ds_account_expires as datetime from "ATTq589983",
    ds_password_last_set as datetime from "ATTq589920",
    ds_bad_pwd_time as datetime from "ATTq589873",
    ds_logon_count as i32 from "ATTj589993",
    ds_bad_pwd_count as i32 from "ATTj589836",
    ds_primary_group_id as i32 from "ATTj589922",
    // ds_unix_password as i32 from "ATTk591734",
    ds_aduser_objects as binary from "ATTk36",
    ds_att_comment as str from "ATTm13",
    
    ds_dns_host_name as str from "ATTm590443",
    ds_os_name as str from "ATTm590187",
    ds_os_version as str from "ATTm590188",
    
    // DS_RECOVERY_PASSWORD as i32 from "ATTm591788",
    // DS_FVEKEY_PACKAGE as i32 from "ATTk591823",
    // DS_VOLUME_GUID as i32 from "ATTk591822",
    // DS_RECOVERY_GUID as i32 from "ATTk591789",
    // DS_DIAL_IN_ACCESS_PERMISSION_NAME as i32 from "ATTi590943",
    // DS_PEK as i32 from "ATTk590689",
);

pub(crate) trait RecordToBodyfile {
    fn to_bodyfile(&self, mapping: &ColumnInfoMapping, type_name: &str) -> Result<Vec<Bodyfile3Line>>;
}

macro_rules! add_bodyfile_timestamp {
    ($res: tt, $field: expr, $object_name: expr, $type_name: expr, $caption: expr) => {
        if let Some(ts) = $field {
            if ts.timestamp() > 0 {
                $res.push(
                    Bodyfile3Line::new()
                        .with_owned_name(format!("{} ({}, {})", $object_name, $type_name, $caption))
                        .with_crtime(i64::max(0,ts.timestamp()))
                );
            }
        }
    };
}

impl RecordToBodyfile for DbRecord<'_> {
    fn to_bodyfile(&self, mapping: &ColumnInfoMapping, type_name: &str) -> Result<Vec<Bodyfile3Line>> {
        let mut res = Vec::new();

        let object_name = self.ds_object_name(mapping)?
            .or(self.ds_object_name2(mapping)?)
            .unwrap_or_else(|| "unknown".to_owned());

        add_bodyfile_timestamp!(res, self.ds_record_time(mapping)?, object_name, type_name, "record creation time");
        add_bodyfile_timestamp!(res, self.ds_when_created(mapping)?, object_name, type_name, "object created");
        add_bodyfile_timestamp!(res, self.ds_when_changed(mapping)?, object_name, type_name, "object changed");
        add_bodyfile_timestamp!(res, self.ds_last_logon(mapping)?, object_name, type_name, "last logon on this DC");
        add_bodyfile_timestamp!(res, self.ds_last_logon_time_stamp(mapping)?, object_name, type_name, "last logon on any DC");
        add_bodyfile_timestamp!(res, self.ds_bad_pwd_time(mapping)?, object_name, type_name, "bad pwd time");
        add_bodyfile_timestamp!(res, self.ds_password_last_set(mapping)?, object_name, type_name, "password last set");

        Ok(res)
    }
}

pub (crate) trait IsMemberOf {
    fn member_of(&self) -> Vec<Box<dyn HasMembers>>;
}

pub (crate) trait HasMembers {
    fn members(&self) -> Vec<Box<dyn IsMemberOf>>;
}

impl IsMemberOf for DbRecord<'_> {
    fn member_of(&self) -> Vec<Box<dyn HasMembers>> {
        todo!()
    }
}