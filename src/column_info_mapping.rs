use crate::{esedb_cache::{CRecord, CDataTable}, column_information::ColumnInformation, win32_types::*, DataTableExt, esedb_utils::*};
use anyhow::Result;
use bodyfile::Bodyfile3Line;
use libesedb::{Record, Value};
use paste::paste;
use std::{collections::HashMap};
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
};

macro_rules! define_getter_int {
    ($field: ident, $res_type: ident) => {
        #[allow(dead_code)]
        pub fn $field(&self, mapping: &ColumnInfoMapping) -> Result<Option<$res_type>> {
            $res_type::from_value_opt(
                self.inner_record.value(mapping.$field.id().try_into().unwrap()).unwrap(),
                stringify!($field)
            )
        }

        paste! {
            #[allow(dead_code)]
            pub fn [<has_valid_ $field>](&self, mapping: &ColumnInfoMapping) -> bool {
                match self.inner_record.value(mapping.$field.id().try_into().unwrap()) {
                    Some(value) => match $res_type::from_value_opt(value, stringify!($field)) {
                        Ok(Some(_)) => true,
                        _ => false
                    }
                    _ => false
                }
            }
        }

        paste! {
            #[allow(dead_code)]
            pub fn [<value_of_ $field>](&self, mapping: &ColumnInfoMapping) -> Option<&Value> {
                self.inner_record.value(mapping.$field.id().try_into().unwrap())
            }
        }

        paste! {
            #[allow(dead_code)]
            pub fn [<$field _equals>](&self, mapping: &ColumnInfoMapping, value: &$res_type) -> bool {
              match self.$field(mapping) {
                Ok(Some(ref v)) => v == value,
                _ => false
              }
            }
        }
    };
}

macro_rules! define_getter {
    ($field: ident as i32) => {
        define_getter_int!($field, i32);
    };
    ($field: ident as u32) => {
        define_getter_int!($field, u32);
    };
    ($field: ident as str) => {
        define_getter_int!($field, String);
    };
    ($field: ident as sid) => {
        define_getter_int!($field, Sid);
    };
    ($field: ident as binary) => {
        define_getter_int!($field, String);
    };
    ($field: ident as database_time) => {
        define_getter_int!($field, DatabaseTime);
    };
    ($field: ident as windows_file_time) => {
        define_getter_int!($field, WindowsFileTime);
    };
    ($field: ident as truncated_windows_file_time) => {
        define_getter_int!($field, TruncatedWindowsFileTime);
    };
    ($field: ident as uac_flags) => {
        define_getter_int!($field, UserAccountControl);
    };
    ($field: ident as sam_account_type) => {
        define_getter_int!($field, SamAccountType);
    };
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
                pub fn from(data_table: &CDataTable) -> Result<Self> {
                    let mut temporary_mapping = HashMap::new();
                    let mut column_names = HashMap::new();
                    for index in 0..data_table.count_columns() {
                        let column_res = data_table.column(index).unwrap();
                        let col_info = ColumnInformation::new(
                            index,
                            // column_res.name()?,
                            // column_res.variant()?
                        );
                        column_names.insert(index, column_res.name().to_owned());
                        temporary_mapping.insert(column_res.name(), col_info);
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

            pub(crate) struct $RecordStructName {
                inner_record: CRecord,
            }

            impl $RecordStructName {
                $(
                    define_getter!($field as $type);
                )+
                pub fn all_attributes(&self, mapping: &$StructName) -> HashMap<String, String> {
                    let mut attribs = HashMap::new();
                    for index in 0..self.inner_record.count_values() {
                        if let Some(value) = self.inner_record.value(index) {
                            if ! matches!(value, Value::Null(())) {
                                if let Some(column_name) = mapping.name_of_column(&index.try_into().unwrap()) {
                                    let str_value = match value {
                                        Value::Null(()) => panic!("unreachable code executed"),
                                        Value::Bool(v) => format!("{v}"),
                                        Value::U8(v) => format!("{v}"),
                                        Value::U16(v) => format!("{v}"),
                                        Value::U32(v) => format!("{v}"),
                                        Value::I16(v) => format!("{v}"),
                                        Value::I32(v) => format!("{v}"),
                                        Value::I64(v) => format!("{v}"),
                                        Value::F32(v) => format!("{v}"),
                                        Value::F64(v) => format!("{v}"),
                                        Value::Currency(v) => format!("{v}"),
                                        Value::DateTime(v) => format!("{v}"),
                                        Value::Binary(v) => hex::encode(&v).to_string(),
                                        Value::Text(v) => v.to_string(),
                                        Value::LargeBinary(v) => hex::encode(&v).to_string(),
                                        Value::LargeText(v) => v.clone(),
                                        Value::SuperLarge(v) => hex::encode(&v).to_string(),
                                        Value::Guid(v) => hex::encode(&v).to_string(),
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
                fn from(dbrecord: &$RecordStructName, data_table: &DataTableExt) -> Result<Self>;
            }

            impl From<CRecord> for $RecordStructName {
                fn from(inner: CRecord) -> Self {
                    Self {
                        inner_record: inner
                    }
                }
            }

            impl<'r> TryFrom<Record<'r>> for $RecordStructName {
                type Error = std::io::Error;
                fn try_from(record: Record<'r>) -> Result<Self, Self::Error> {
                    Ok(Self::from(CRecord::try_from(record)?))
                }
            }
        }
    }

// This mapping should be defined in `%WINDIR%\ntds\schema.ini`
column_mapping! (
    ColumnInfoMapping {
        column_names: HashMap<i32, String>,
    },
    DbRecord,
    ds_record_id as i32 from "DNT_col",
    ds_parent_record_id as i32 from "PDNT_col",
    ds_record_time as truncated_windows_file_time from "time_col",
    ds_ancestors as i32 from "Ancestors_col",
    ds_object_type_id as i32 from "ATTb590606",
    ds_object_name as str from "ATTm3",
    ds_object_name2 as str from "ATTm589825",
    // DS_OBJECT_GUID as i32 from "ATTk589826",
    ds_when_created as truncated_windows_file_time from "ATTl131074",
    ds_when_changed as truncated_windows_file_time from "ATTl131075",
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
    ds_last_logon as windows_file_time from "ATTq589876",
    ds_last_logon_time_stamp as windows_file_time from "ATTq591520",
    ds_account_expires as windows_file_time from "ATTq589983",
    ds_password_last_set as windows_file_time from "ATTq589920",
    ds_bad_pwd_time as windows_file_time from "ATTq589873",
    ds_logon_count as i32 from "ATTj589993",
    ds_bad_pwd_count as i32 from "ATTj589836",
    ds_primary_group_id as i32 from "ATTj589922",
    // ds_unix_password as i32 from "ATTk591734",
    ds_aduser_objects as binary from "ATTk36",
    ds_att_comment as str from "ATTm13",

    ds_dns_host_name as str from "ATTm590443",
    ds_os_name as str from "ATTm590187",
    ds_os_version as str from "ATTm590188",

    ds_link_id as u32 from "ATTj131122",
    ds_ldap_display_name as str from "ATTm131532",

    // DS_RECOVERY_PASSWORD as i32 from "ATTm591788",
    // DS_FVEKEY_PACKAGE as i32 from "ATTk591823",
    // DS_VOLUME_GUID as i32 from "ATTk591822",
    // DS_RECOVERY_GUID as i32 from "ATTk591789",
    // DS_DIAL_IN_ACCESS_PERMISSION_NAME as i32 from "ATTi590943",
    // DS_PEK as i32 from "ATTk590689",

    ds_created_sid as sid from "ATTr591234",
);

pub(crate) trait FormatDbRecordForCli {
    fn to_table(&self, mapping: &ColumnInfoMapping) -> term_table::Table;
}

impl FormatDbRecordForCli for DbRecord {
    fn to_table(&self, mapping: &ColumnInfoMapping) -> term_table::Table {
        let mut table = term_table::Table::new();
        let all_attributes = self.all_attributes(mapping);
        let mut keys = all_attributes.keys().collect::<Vec<&String>>();
        keys.sort();

        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Attribute", 1, Alignment::Center),
            TableCell::new_with_alignment("Value", 1, Alignment::Center),
        ]));

        for key in keys {
            table.add_row(Row::new(vec![
                TableCell::new(key),
                TableCell::new(all_attributes[key].to_string()),
            ]));
        }

        table
    }
}

pub(crate) trait RecordToBodyfile {
    fn to_bodyfile(
        &self,
        mapping: &ColumnInfoMapping,
        type_name: &str,
    ) -> Result<Vec<Bodyfile3Line>>;
}

macro_rules! add_bodyfile_timestamp {
    ($res: tt, $field: expr, $object_name: expr, $type_name: expr, $caption: expr) => {
        if let Some(ts) = $field {
            if ts.timestamp() > 0 {
                $res.push(
                    Bodyfile3Line::new()
                        .with_owned_name(format!("{} ({}, {})", $object_name, $type_name, $caption))
                        .with_crtime(i64::max(0, ts.timestamp())),
                );
            }
        }
    };
}

impl RecordToBodyfile for DbRecord {
    fn to_bodyfile(
        &self,
        mapping: &ColumnInfoMapping,
        type_name: &str,
    ) -> Result<Vec<Bodyfile3Line>> {
        let mut res = Vec::new();

        let object_name = self
            .ds_object_name(mapping)?
            .or(self.ds_object_name2(mapping)?)
            .unwrap_or_else(|| "unknown".to_owned());

        add_bodyfile_timestamp!(
            res,
            self.ds_record_time(mapping)?,
            object_name,
            type_name,
            "record creation time"
        );
        add_bodyfile_timestamp!(
            res,
            self.ds_when_created(mapping)?,
            object_name,
            type_name,
            "object created"
        );
        add_bodyfile_timestamp!(
            res,
            self.ds_when_changed(mapping)?,
            object_name,
            type_name,
            "object changed"
        );
        add_bodyfile_timestamp!(
            res,
            self.ds_last_logon(mapping)?,
            object_name,
            type_name,
            "last logon on this DC"
        );
        add_bodyfile_timestamp!(
            res,
            self.ds_last_logon_time_stamp(mapping)?,
            object_name,
            type_name,
            "last logon on any DC"
        );
        add_bodyfile_timestamp!(
            res,
            self.ds_bad_pwd_time(mapping)?,
            object_name,
            type_name,
            "bad pwd time"
        );
        add_bodyfile_timestamp!(
            res,
            self.ds_password_last_set(mapping)?,
            object_name,
            type_name,
            "password last set"
        );

        Ok(res)
    }
}

pub(crate) trait IsMemberOf {
    fn member_of(&self) -> Vec<Box<dyn HasMembers>>;
}

pub(crate) trait HasMembers {
    fn members(&self) -> Vec<Box<dyn IsMemberOf>>;
}

impl IsMemberOf for DbRecord {
    fn member_of(&self) -> Vec<Box<dyn HasMembers>> {
        todo!()
    }
}
