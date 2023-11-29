use std::collections::HashMap;

use crate::cache;
use crate::ntds::{Error, NtdsAttributeId, Result};
use crate::value::FromValue;
use crate::win32_types::{Sid, TruncatedWindowsFileTime, SamAccountType, UserAccountControl, WindowsFileTime};
use crate::ColumnInfoMapping;
use concat_idents::concat_idents;
use dashmap::mapref::one::RefMut;
use libesedb::Value;
use term_table::row::Row;
use term_table::table_cell::{Alignment, TableCell};

pub struct DataTableRecord<'info, 'db>(&'db cache::Record<'info, 'db>);

impl<'info, 'db> From<&'db cache::Record<'info, 'db>> for DataTableRecord<'info, 'db> {
    fn from(record: &'db cache::Record<'info, 'db>) -> Self {
        Self(record)
    }
}

macro_rules! record_attribute {
    ($name: ident, $id: ident, $type: ty) => {
        pub fn $name(&self) -> Result<$type> {
            <$type>::from_value(
                self.0
                    .get_by_id(NtdsAttributeId::$id)
                    .ok_or(Error::ValueIsMissing)?
                    .value(),
            )
        }
        concat_idents!(fn_name = $name, _opt {
            pub fn fn_name (&self) -> Result<Option<$type>> {
                <$type>::from_value_opt(
                    self.0
                        .get_by_id(NtdsAttributeId::$id)
                        .ok_or(Error::ValueIsMissing)?
                        .value(),
                )
            }
        });
    };
}

impl<'info, 'db> DataTableRecord<'info, 'db> {
    record_attribute!(ds_object_sid, AttObjectSid, Sid);
    record_attribute!(ds_record_id, DsRecordId, i32);
    record_attribute!(ds_parent_record_id, DsParentRecordId, i32);
    record_attribute!(ds_record_time, DsRecordTime, TruncatedWindowsFileTime);
    record_attribute!(ds_ancestors, DsAncestors, i32);
    record_attribute!(ds_object_type_id, AttObjectCategory, i32);
    record_attribute!(ds_object_name, AttCommonName, String);
    record_attribute!(ds_object_name2, AttRdn, String);
    record_attribute!(ds_sam_account_name, AttSamAccountName, String);
    record_attribute!(ds_sam_account_type, AttSamAccountType, SamAccountType);
    record_attribute!(ds_user_principal_name, AttUserPrincipalName, String);
    record_attribute!(ds_user_account_control, AttUserAccountControl, UserAccountControl);
    record_attribute!(ds_last_logon, AttLastLogon, WindowsFileTime);
    record_attribute!(ds_last_logon_time_stamp, AttLastLogonTimestamp, WindowsFileTime);
    record_attribute!(ds_account_expires, AttAccountExpires, WindowsFileTime);
    record_attribute!(ds_password_last_set, AttPwdLastSet, WindowsFileTime);
    record_attribute!(ds_bad_pwd_time, AttBadPasswordTime, WindowsFileTime);
    record_attribute!(ds_logon_count, AttLogonCount, i32);
    record_attribute!(ds_bad_pwd_count, AttBadPwdCount, i32);
    record_attribute!(ds_primary_group_id, AttPrimaryGroupId, i32);
    //record_attribute!(ds_aduser_objects, AttX509Cert, Vec<u8>);
    record_attribute!(ds_att_comment, AttComment, String);
    record_attribute!(ds_dns_host_name, AttDnsHostName, String);
    record_attribute!(ds_os_name, AttOperatingSystem, String);
    record_attribute!(ds_os_version, AttOperatingSystemVersion, String);
    record_attribute!(ds_link_id, AttLinkId, u32);
    record_attribute!(ds_ldap_display_name, AttLdapDisplayName, String);
    record_attribute!(ds_creator_sid, AttMsDsCreatorSid, Sid);
    record_attribute!(ds_admin_count, AttAdminCount, i32);

    pub fn get(&self, attribute_id: NtdsAttributeId) -> Option<RefMut<'_, i32, Value>> {
        self.0.get_by_id(attribute_id)
    }
    pub fn get_by_index(&self, index: i32) -> Option<RefMut<'_, i32, Value>> {
        self.0.get_by_index(index)
    }

    pub fn mapping(&self) -> &ColumnInfoMapping {
        self.0.esedbinfo().mapping()
    }
    pub fn all_attributes(&self) -> HashMap<String, String> {
        self.0
            .values()
            .iter()
            .map(|m| {
                (
                    self.0.columns()[*m.key() as usize].name().to_owned(),
                    m.value().to_string(),
                )
            })
            .collect()
    }
}

impl<'info, 'db> From<&DataTableRecord<'info, 'db>> for term_table::Table<'_> {
    fn from(value: &DataTableRecord<'info, 'db>) -> Self {
        let mut table = term_table::Table::new();
        let all_attributes = value.all_attributes();
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
