use crate::cache::{self, MetaDataCache, RecordId, RecordPointer};
use crate::cache::{ColumnIndex, WithValue};
use crate::ntds::{Error, NtdsAttributeId};
use crate::value::FromValue;
use crate::win32_types::TimelineEntry;
use crate::win32_types::{
    Rdn, SamAccountType, Sid, TruncatedWindowsFileTime, UserAccountControl, WindowsFileTime,
};
use crate::ColumnInfoMapping;
use bodyfile::Bodyfile3Line;
use chrono::{DateTime, Utc};
use concat_idents::concat_idents;
use flow_record::derive::*;
use flow_record::prelude::*;
use getset::Getters;
use serde::ser::SerializeStruct;
use serde::Serialize;
use std::collections::HashMap;
use term_table::row::Row;
use term_table::table_cell::{Alignment, TableCell};

use super::{AttributeName, AttributeValue};

#[derive(Getters, Serialize)]
#[getset(get = "pub")]
pub struct EntryAttribute {
    column: String,
    attribute: AttributeName,
    value: AttributeValue,
}

#[derive(Getters)]
pub struct DataTableRecord<'info, 'db> {
    inner: cache::Record<'info, 'db>,

    #[getset(get = "pub")]
    ptr: RecordPointer,
}

macro_rules! record_attribute {
    ($name: ident, $id: ident, $type: ty) => {
        pub fn $name(&self) -> anyhow::Result<$type> {
            self.get_value(NtdsAttributeId::$id)
        }

        concat_idents!(fn_name=$name, _opt {
            pub fn fn_name(&self) -> anyhow::Result<Option<$type>> {
                self.get_value_opt(NtdsAttributeId::$id)
            }
        });

        concat_idents!(fn_name=has_, $name {
            pub fn fn_name(&self, other: &$type) -> anyhow::Result<bool> {
                self.has_value(NtdsAttributeId::$id, other)
            }
        });
    };
}

impl<'info, 'db> DataTableRecord<'info, 'db> {
    pub fn new(inner: cache::Record<'info, 'db>, ptr: RecordPointer) -> Self {
        Self { inner, ptr }
    }

    fn get_value<T>(&self, column: NtdsAttributeId) -> anyhow::Result<T>
    where
        T: FromValue,
    {
        self.inner.with_value(column, |v| match v {
            None => Err(anyhow::anyhow!(Error::ValueIsMissing)),
            Some(v) => Ok(<T>::from_value(v)?),
        })
    }
    fn get_value_opt<T>(&self, column: NtdsAttributeId) -> anyhow::Result<Option<T>>
    where
        T: FromValue,
    {
        self.inner.with_value(column, |v| match v {
            None => Ok(None),
            Some(v) => Ok(Some(<T>::from_value(v)?)),
        })
    }
    fn has_value<T>(&self, column: NtdsAttributeId, other: &T) -> anyhow::Result<bool>
    where
        T: FromValue + Eq,
    {
        self.inner.with_value(column, |v| match v {
            None => Ok(false),
            Some(v) => Ok(&(<T>::from_value(v)?) == other),
        })
    }

    record_attribute!(ds_record_id, DsRecordId, RecordId);
    record_attribute!(object_category, AttObjectCategory, RecordId);
    record_attribute!(ds_parent_record_id, DsParentRecordId, RecordId);
    record_attribute!(ds_record_time, DsRecordTime, TruncatedWindowsFileTime);
    record_attribute!(ds_ancestors, DsAncestors, i32);
    record_attribute!(att_object_sid, AttObjectSid, Sid);
    record_attribute!(att_when_created, AttWhenCreated, TruncatedWindowsFileTime);
    record_attribute!(att_when_changed, AttWhenChanged, TruncatedWindowsFileTime);
    record_attribute!(att_object_type_id, AttObjectCategory, RecordId);
    record_attribute!(att_object_name, AttCommonName, Rdn);
    record_attribute!(att_object_name2, AttRdn, Rdn);
    record_attribute!(att_sam_account_name, AttSamAccountName, String);
    record_attribute!(att_sam_account_type, AttSamAccountType, SamAccountType);
    record_attribute!(att_user_principal_name, AttUserPrincipalName, String);
    record_attribute!(
        att_user_account_control,
        AttUserAccountControl,
        UserAccountControl
    );
    record_attribute!(att_last_logon, AttLastLogon, WindowsFileTime);
    record_attribute!(
        att_last_logon_time_stamp,
        AttLastLogonTimestamp,
        WindowsFileTime
    );
    record_attribute!(att_account_expires, AttAccountExpires, WindowsFileTime);
    record_attribute!(att_password_last_set, AttPwdLastSet, WindowsFileTime);
    record_attribute!(att_bad_pwd_time, AttBadPasswordTime, WindowsFileTime);
    record_attribute!(att_logon_count, AttLogonCount, i32);
    record_attribute!(att_bad_pwd_count, AttBadPwdCount, i32);
    record_attribute!(att_primary_group_id, AttPrimaryGroupId, i32);
    //record_attribute!(att_aduser_objects, AttX509Cert, Vec<u8>);
    record_attribute!(att_comment, AttComment, String);
    record_attribute!(att_dns_host_name, AttDnsHostName, String);
    record_attribute!(att_os_name, AttOperatingSystem, String);
    record_attribute!(att_os_version, AttOperatingSystemVersion, String);
    record_attribute!(att_link_id, AttLinkId, u32);
    record_attribute!(att_ldap_display_name, AttLdapDisplayName, String);
    record_attribute!(att_creator_sid, AttMsDsCreatorSid, Sid);
    record_attribute!(att_admin_count, AttAdminCount, i32);
    record_attribute!(att_is_deleted, AttIsDeleted, bool);
    record_attribute!(att_last_known_parent, AttLastKnownParent, RecordId);
    record_attribute!(att_nt_security_descriptor, AttNtSecurityDescriptor, i64);

    pub fn mapping(&self) -> &ColumnInfoMapping {
        self.inner.esedbinfo().mapping()
    }
    pub fn all_attributes(&self) -> HashMap<NtdsAttributeId, EntryAttribute> {
        (0..*self.inner.count())
            .map(ColumnIndex::from)
            .filter_map(|idx| {
                let column = &self.inner.columns()[idx];
                if column.attribute_id().is_some() {
                    Some(column)
                } else {
                    None
                }
            })
            .map(|column| {
                self.inner.with_value(*column.index(), |v| {
                    Ok(v.map(|x| {
                        (
                            column.attribute_id().unwrap(),
                            EntryAttribute {
                                column: column.name().to_string(),
                                attribute: column
                                    .attribute_name()
                                    .as_ref()
                                    .cloned()
                                    .unwrap_or(AttributeName::from(column.name().to_string())),
                                value: AttributeValue::from(x.to_string()),
                            },
                        )
                    }))
                })
            })
            .filter_map(Result::ok)
            .flatten()
            .collect()
    }

    pub fn object_type_name(&self, metadata: &MetaDataCache) -> anyhow::Result<String> {
        Ok(if let Some(type_id) = self.att_object_type_id_opt()? {
            metadata
                .record(&type_id)
                .map(|entry| entry.rdn().name().to_string())
                .unwrap_or("Object".to_string())
        } else {
            "Object".to_string()
        })
    }

    pub fn to_bodyfile(&self, metadata: &MetaDataCache) -> anyhow::Result<Vec<Bodyfile3Line>> {
        let my_name = self
            .att_sam_account_name()
            .or(self.att_object_name().map(|s| s.name().to_string()));

        let object_type_name = self.object_type_name(metadata)?;
        let object_type_caption =
            if let Some(last_known_parent) = self.att_last_known_parent_opt()? {
                metadata
                    .record(&last_known_parent)
                    .and_then(|entry| metadata.dn(entry))
                    .map(|e| format!("{object_type_name}, deleted from {e}"))
                    .unwrap_or(format!("deleted {object_type_name}"))
            } else if self.att_is_deleted_opt()?.unwrap_or(false) {
                format!("deleted {object_type_name}")
            } else {
                object_type_name
            };

        let inode = self.ptr.ds_record_id().to_string();
        if let Ok(upn) = &my_name {
            Ok(vec![
                self.ds_record_time().map(|ts| {
                    ts.cr_entry(upn, "record creation time", &object_type_caption)
                        .with_inode(&inode)
                }),
                self.att_when_created().map(|ts| {
                    ts.cr_entry(upn, "object created", &object_type_caption)
                        .with_inode(&inode)
                }),
                self.att_when_changed().map(|ts| {
                    ts.cr_entry(upn, "object changed", &object_type_caption)
                        .with_inode(&inode)
                }),
                self.att_last_logon().map(|ts| {
                    ts.c_entry(upn, "last logon on this DC", &object_type_caption)
                        .with_inode(&inode)
                }),
                self.att_last_logon_time_stamp().map(|ts| {
                    ts.c_entry(upn, "last logon on any DC", &object_type_caption)
                        .with_inode(&inode)
                }),
                self.att_bad_pwd_time().map(|ts| {
                    ts.c_entry(upn, "bad pwd time", &object_type_caption)
                        .with_inode(&inode)
                }),
                self.att_password_last_set().map(|ts| {
                    ts.c_entry(upn, "password last set", object_type_caption)
                        .with_inode(&inode)
                }),
            ]
            .into_iter()
            .flatten()
            .collect())
        } else {
            Ok(Vec::new())
        }
    }
    pub fn to_flow_record(&self, metadata: &MetaDataCache) -> anyhow::Result<NtdsEntry> {
        let name = self
            .att_sam_account_name()
            .or(self.att_object_name().map(|s| s.name().to_string()))?;

        let object_type = self.object_type_name(metadata)?;
        let deleted_from = self
            .att_last_known_parent_opt()?
            .and_then(|last_known_parent| {
                metadata
                    .record(&last_known_parent)
                    .and_then(|entry| metadata.dn(entry))
            });

        Ok(NtdsEntry {
            name,
            object_type,
            record_id: self.ptr.ds_record_id().inner(),
            is_deleted: self.att_is_deleted_opt()?.unwrap_or(false),
            deleted_from,
            record_time: self.ds_record_time_opt()?.map(|ts| ts.into()),
            when_created: self.att_when_created_opt()?.map(|ts| ts.into()),
            when_changed: self.att_when_changed_opt()?.map(|ts| ts.into()),
            last_logon: self.att_last_logon_opt()?.map(|ts| ts.into()),
            last_logon_timestamp: self.att_last_logon_time_stamp_opt()?.map(|ts| ts.into()),
            bad_pwd_time: self.att_bad_pwd_time_opt()?.map(|ts| ts.into()),
            password_last_set: self.att_password_last_set_opt()?.map(|ts| ts.into()),
        })
    }
}

impl<'info, 'db> WithValue<NtdsAttributeId> for DataTableRecord<'info, 'db> {
    fn with_value<T>(
        &self,
        index: NtdsAttributeId,
        function: impl FnMut(Option<&cache::Value>) -> anyhow::Result<T>,
    ) -> anyhow::Result<T> {
        self.inner.with_value(index, function)
    }
}

impl<'info, 'db> WithValue<ColumnIndex> for DataTableRecord<'info, 'db> {
    fn with_value<T>(
        &self,
        index: ColumnIndex,
        function: impl FnMut(Option<&cache::Value>) -> anyhow::Result<T>,
    ) -> anyhow::Result<T> {
        self.inner.with_value(index, function)
    }
}

impl<'info, 'db> From<&DataTableRecord<'info, 'db>> for term_table::Table {
    fn from(value: &DataTableRecord<'info, 'db>) -> Self {
        let mut table = term_table::Table::new();
        let all_attributes = value.all_attributes();
        let mut keys: Vec<_> = all_attributes.keys().collect();
        keys.sort();

        table.add_row(Row::new(vec![
            TableCell::builder("Attribute")
                .alignment(Alignment::Center)
                .build(),
            TableCell::builder("Value")
                .alignment(Alignment::Center)
                .build(),
        ]));

        for id in keys {
            let attribute = &all_attributes[id];
            table.add_row(Row::new(vec![
                TableCell::new(attribute.attribute()),
                TableCell::new(attribute.column()),
                TableCell::new(attribute.value()),
            ]));
        }

        table
    }
}

impl<'info, 'db> Serialize for DataTableRecord<'info, 'db> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let all_attributes = self.all_attributes();
        let mut ser = serializer.serialize_struct("record", all_attributes.len())?;
        for (id, att) in all_attributes {
            let key: &'static str = id.into();
            ser.serialize_field(key, att.value())?;
        }
        ser.end()
    }
}

#[derive(FlowRecord)]
#[flow_record(version = 1, source = "ntdsextract2", classification = "ntds")]
pub struct NtdsEntry {
    name: String,
    object_type: String,
    record_id: i32,
    record_time: Option<DateTime<Utc>>,
    when_created: Option<DateTime<Utc>>,
    when_changed: Option<DateTime<Utc>>,
    last_logon: Option<DateTime<Utc>>,
    last_logon_timestamp: Option<DateTime<Utc>>,
    bad_pwd_time: Option<DateTime<Utc>>,
    password_last_set: Option<DateTime<Utc>>,
    is_deleted: bool,
    deleted_from: Option<String>,
}
