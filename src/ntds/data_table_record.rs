use crate::cache::{self, EsedbRowId, RecordId};
use crate::cache::{ColumnIndex, Value, WithValue};
use crate::ntds::{Error, NtdsAttributeId};
use crate::value::FromValue;
use crate::win32_types::TimelineEntry;
use crate::win32_types::{
    Rdn, SamAccountType, Sid, TruncatedWindowsFileTime, UserAccountControl, WindowsFileTime,
};
use crate::ColumnInfoMapping;
use bodyfile::Bodyfile3Line;
use concat_idents::concat_idents;
use std::collections::HashMap;
use term_table::row::Row;
use term_table::table_cell::{Alignment, TableCell};

use super::{AttributeName, AttributeValue};

pub struct DataTableRecord<'info, 'db> {
    inner: cache::Record<'info, 'db>,
    _row: EsedbRowId,
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
    pub fn new(inner: cache::Record<'info, 'db>, row: EsedbRowId) -> Self {
        Self { inner, _row: row }
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
    record_attribute!(att_object_name, AttCommonName, String);
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

    pub fn mapping(&self) -> &ColumnInfoMapping {
        self.inner.esedbinfo().mapping()
    }
    pub fn all_attributes(
        &self,
    ) -> HashMap<NtdsAttributeId, (String, AttributeName, AttributeValue)> {
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
                            (
                                column.name().to_string(),
                                column
                                    .attribute_name()
                                    .as_ref().cloned()
                                    .unwrap_or(AttributeName::from(column.name().to_string())),
                                AttributeValue::from(x.to_string()),
                            ),
                        )
                    }))
                })
            })
            .filter_map(Result::ok)
            .flatten()
            .collect()
    }
}

impl<'info, 'db> WithValue<NtdsAttributeId> for DataTableRecord<'info, 'db> {
    fn with_value<T>(
        &self,
        index: NtdsAttributeId,
        function: impl FnMut(Option<&Value>) -> anyhow::Result<T>,
    ) -> anyhow::Result<T> {
        self.inner.with_value(index, function)
    }
}

impl<'info, 'db> WithValue<ColumnIndex> for DataTableRecord<'info, 'db> {
    fn with_value<T>(
        &self,
        index: ColumnIndex,
        function: impl FnMut(Option<&Value>) -> anyhow::Result<T>,
    ) -> anyhow::Result<T> {
        self.inner.with_value(index, function)
    }
}

impl<'info, 'db> From<&DataTableRecord<'info, 'db>> for term_table::Table<'_> {
    fn from(value: &DataTableRecord<'info, 'db>) -> Self {
        let mut table = term_table::Table::new();
        let all_attributes = value.all_attributes();
        let mut keys: Vec<_> = all_attributes.keys().collect();
        keys.sort();

        table.add_row(Row::new(vec![
            TableCell::new_with_alignment("Attribute", 1, Alignment::Center),
            TableCell::new_with_alignment("Value", 1, Alignment::Center),
        ]));

        for id in keys {
            let (col_name, att_name, value) = &all_attributes[id];
            table.add_row(Row::new(vec![
                TableCell::new(att_name),
                TableCell::new(col_name),
                TableCell::new(value),
            ]));
        }

        table
    }
}

impl<'info, 'db> TryFrom<DataTableRecord<'info, 'db>> for Vec<Bodyfile3Line> {
    type Error = anyhow::Error;

    fn try_from(obj: DataTableRecord) -> core::result::Result<Self, Self::Error> {
        let my_name = obj.att_sam_account_name().or(obj.att_object_name());
        if let Ok(upn) = &my_name {
            Ok(vec![
                obj.ds_record_time()
                    .map(|ts| ts.cr_entry(upn, "record creation time", "object")),
                obj.att_when_created()
                    .map(|ts| ts.cr_entry(upn, "object created", "object")),
                obj.att_when_changed()
                    .map(|ts| ts.cr_entry(upn, "object changed", "object")),
                obj.att_last_logon()
                    .map(|ts| ts.c_entry(upn, "last logon on this DC", "object")),
                obj.att_last_logon_time_stamp()
                    .map(|ts| ts.c_entry(upn, "last logon on any DC", "object")),
                obj.att_bad_pwd_time()
                    .map(|ts| ts.c_entry(upn, "bad pwd time", "object")),
                obj.att_password_last_set()
                    .map(|ts| ts.c_entry(upn, "password last set", "object")),
            ]
            .into_iter()
            .flatten()
            .collect())
        } else {
            Ok(Vec::new())
        }
    }
}
