use std::collections::HashMap;
use anyhow::Result;
use ntds_object_derive::NTDSObject;
use crate::win32_types::{SamAccountType, Sid, TruncatedWindowsFileTime, UserAccountControl, WindowsFileTime};

#[derive(NTDSObject)]
pub struct DBRecord {
    #[ntds(attribute=DsRecordId)]                ds_record_id: i32,
    #[ntds(attribute=DsParentRecordId)]          ds_parent_record_id: i32,
    #[ntds(attribute=DsRecordTime)]              ds_record_time: TruncatedWindowsFileTime,
    #[ntds(attribute=DsAncestors)]               ds_ancestors: i32,
    #[ntds(attribute=AttObjectCategory)]         ds_object_type_id: i32,
    #[ntds(attribute=AttCommonName)]             ds_object_name: String,
    #[ntds(attribute=AttRdn)]                    ds_object_name2: String,
    #[ntds(attribute=AttWhenCreated)]            ds_when_created: TruncatedWindowsFileTime,
    #[ntds(attribute=AttWhenChanged)]            ds_when_changed: TruncatedWindowsFileTime,
    #[ntds(attribute=AttObjectSid)]              ds_sid: Sid,
    #[ntds(attribute=AttSamAccountName)]         ds_sam_account_name: String,
    #[ntds(attribute=AttUserPrincipalName)]      ds_user_principal_name: String,
    #[ntds(attribute=AttSamAccountType)]         ds_sam_account_type: SamAccountType,
    #[ntds(attribute=AttUserAccountControl)]     ds_user_account_control: UserAccountControl,
    #[ntds(attribute=AttLastLogon)]              ds_last_logon: WindowsFileTime,
    #[ntds(attribute=AttLastLogonTimestamp)]     ds_last_logon_time_stamp: WindowsFileTime,
    #[ntds(attribute=AttAccountExpires)]         ds_account_expires: WindowsFileTime,
    #[ntds(attribute=AttPwdLastSet)]             ds_password_last_set: WindowsFileTime,
    #[ntds(attribute=AttBadPasswordTime)]        ds_bad_pwd_time: WindowsFileTime,
    #[ntds(attribute=AttLogonCount)]             ds_logon_count: i32,
    #[ntds(attribute=AttBadPwdCount)]            ds_bad_pwd_count: i32,
    #[ntds(attribute=AttPrimaryGroupId)]         ds_primary_group_id: i32,
    #[ntds(attribute=AttX509Cert)]               ds_aduser_objects: String,
    #[ntds(attribute=AttComment)]                ds_att_comment: String,
    #[ntds(attribute=AttDnsHostName)]            ds_dns_host_name: String,
    #[ntds(attribute=AttOperatingSystem)]        ds_os_name: String,
    #[ntds(attribute=AttOperatingSystemVersion)] ds_os_version: String,
    #[ntds(attribute=AttLinkId)]                 ds_link_id: u32,
    #[ntds(attribute=AttLdapDisplayName)]        ds_ldap_display_name: String,
    #[ntds(attribute=AttMsDsCreatorSid)]         ds_creator_sid: Sid,
    #[ntds(attribute=AttAdminCount)]             ds_admin_count: i32,
    #[ntds(attribute=AttIsDeleted)]              ds_is_deleted: bool,

    // DS_OBJECT_GUID: i32, //ATTk589826",
    // DS_USNCREATED: datetime, //ATTq131091",
    // DS_USNCHANGED: datetime, //ATTq131192",
    // DS_OBJECT_COL: i32, //OBJ_col",
    // DS_IS_DELETED: i32, //ATTi131120",
    // DS_ORIG_CONTAINER_ID: i32, //ATTb590605",
    // ds_unix_password: i32, //ATTk591734",
    // DS_RECOVERY_PASSWORD: i32, //ATTm591788",
    // DS_FVEKEY_PACKAGE: i32, //ATTk591823",
    // DS_VOLUME_GUID: i32, //ATTk591822",
    // DS_RECOVERY_GUID: i32, //ATTk591789",
    // DS_DIAL_IN_ACCESS_PERMISSION_NAME: i32, //ATTi590943",
    // DS_PEK: i32, //ATTk590689",
}


impl<'a> FormatDbRecordForCli for DbRecord<'a> {
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
