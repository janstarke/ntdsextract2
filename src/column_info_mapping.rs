use std::{ops::Index, collections::HashMap};

use crate::{column_information::ColumnInformation, ntds::NtdsAttributeId};
use anyhow::Result;
use bodyfile::Bodyfile3Line;
use libesedb::Table;

pub struct ColumnInfoMapping {
    mapping: HashMap<NtdsAttributeId, ColumnInformation>,
}

impl Index<NtdsAttributeId> for ColumnInfoMapping {
    type Output = Option<i32>;

    fn index(&self, index: NtdsAttributeId) -> &Self::Output {
        &self.mapping.get(&index).map(|info| info.id())
    }
}

impl TryFrom<&Table<'_>> for ColumnInfoMapping {
    type Error = anyhow::Error;
    fn try_from(data_table: &Table) -> Result<Self, Self::Error> {
        let mut mapping = HashMap::new();
        for index in 0..data_table.count_columns()? {
            let column = data_table.column(index)?;
            if let Ok(column_id) = NtdsAttributeId::try_from(&column.name()?[..]) {
                let col_info = ColumnInformation::new(
                    index,
                    // column_res.name()?,
                    // column_res.variant()?
                );
                mapping.insert(column_id, col_info);
            }
            //log::info!("found column with name {name}", name=column_res.name());
        }

        Ok(Self { mapping })
    }
}

macro_rules! define_getter_int {
    ($field: ident, $res_type: ident) => {
        #[allow(dead_code)]
        pub fn $field(&self, mapping: &ColumnInfoMapping) -> Result<Option<$res_type>> {
            let mut value = None;
            self.inner_record.with_value(mapping.$field.id().try_into().unwrap(), |x| value=Some(x));
            match value {
                None => Ok(None),
                Some(v) => $res_type::from_value_opt(&v,stringify!($field))
            }
        }

        paste! {
            #[allow(dead_code)]
            pub fn [<has_valid_ $field>](&self, mapping: &ColumnInfoMapping) -> bool {
                let mut value = None;
                self.inner_record.with_value(mapping.$field.id().try_into().unwrap(), |x| value=Some(x));
                match value {
                    Some(v) => match $res_type::from_value_opt(v, stringify!($field)) {
                        Ok(Some(_)) => true,
                        _ => false
                    }
                    _ => false
                }
            }
        }

        paste! {
            #[allow(dead_code)]
            pub fn [<with_value_of_ $field>]<F>(&self, mapping: &ColumnInfoMapping, action: F) where F:Fn(&Value) {
                self.inner_record.with_value(mapping.$field.id().try_into().unwrap(), action)
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
    ($field: ident as bool) => {
        define_getter_int!($field, bool);
    };
}

macro_rules! column_mapping {
    (
        $StructName:ident { $($manual_fields:tt)* },
        $RecordStructName:ident,
        $($field:ident as $type: ident from $internal_id: expr),+ $(,)?
    ) => {
            #[allow(dead_code)]
            pub struct $StructName {
                $(
                    pub $field: ColumnInformation,
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
                        //log::info!("found column with name {name}", name=column_res.name());
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

            pub struct $RecordStructName<'a> {
                inner_record: CRecord<'a>,
            }

            impl<'a> $RecordStructName<'a> {
                $(
                    define_getter!($field as $type);
                )+
                pub fn all_attributes(&self, mapping: &$StructName) -> HashMap<String, String> {
                    let mut attribs = HashMap::new();
                    for index in 0..self.inner_record.count_values() {
                        self.inner_record.with_value_mut(index, |value| {
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
                        });
                    }
                    attribs
                }
            }

            pub trait FromDbRecord where Self: Sized {
                fn from(dbrecord: &$RecordStructName, database: &CDatabase<'_>) -> Result<Self>;
            }

            impl<'a> From<CRecord<'a>> for $RecordStructName<'a> {
                fn from(inner: CRecord) -> Self {
                    Self {
                        inner_record: inner
                    }
                }
            }

            impl<'r, 'a> TryFrom<Record<'r>> for $RecordStructName<'_> {
                type Error = std::io::Error;
                fn try_from(record: Record<'r>) -> Result<Self, Self::Error> {
                    Ok(Self::from(CRecord::try_from(record)?))
                }
            }

            impl<'a> crate::IsRecord for $RecordStructName<'_> {
                fn with_value_mut<F>(&self, index: i32, mut action: F) where F: FnMut(&Value) {
                    self.inner_record.with_value_mut(index, action)
                }
                fn count_values(&self) -> i32 { self.inner_record.count_values() }
            }
        }
    }

pub trait RecordToBodyfile {
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
/*
impl<'a> RecordToBodyfile for DbRecord<'a> {
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

pub trait IsMemberOf {
    fn member_of(&self) -> Vec<Box<dyn HasMembers>>;
}

pub trait HasMembers {
    fn members(&self) -> Vec<Box<dyn IsMemberOf>>;
}

impl<'a> IsMemberOf for DbRecord<'a> {
    fn member_of(&self) -> Vec<Box<dyn HasMembers>> {
        todo!()
    }
}
 */
