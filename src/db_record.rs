use std::collections::HashMap;
use anyhow::Result;
use ntds_object_derive::NTDSObject;
use crate::win32_types::{SamAccountType, Sid, TruncatedWindowsFileTime, UserAccountControl, WindowsFileTime};

#[derive(NTDSObject)]
pub struct ColumnInfoMapping {
    column_names: HashMap<i32, String>,

    #[ntds_attribute("DNT_col")]    ds_record_id: i32,
    #[ntds_attribute("PDNT_col")]   ds_parent_record_id: i32,
    #[ntds_attribute("time_col")]   ds_record_time: TruncatedWindowsFileTime,
    #[ntds_attribute("Ancestors_col")] ds_ancestors: i32,
    #[ntds_attribute("ATTb590606")] ds_object_type_id: i32,
    #[ntds_attribute("ATTm3")]      ds_object_name: String,
    #[ntds_attribute("ATTm589825")] ds_object_name2: String,
    #[ntds_attribute("ATTl131074")] ds_when_created: TruncatedWindowsFileTime,
    #[ntds_attribute("ATTl131075")] ds_when_changed: TruncatedWindowsFileTime,
    #[ntds_attribute("ATTr589970")] ds_sid: Sid,
    #[ntds_attribute("ATTm590045")] ds_sam_account_name: String,
    #[ntds_attribute("ATTm590480")] ds_user_principal_name: String,
    #[ntds_attribute("ATTj590126")] ds_sam_account_type: SamAccountType,
    #[ntds_attribute("ATTj589832")] ds_user_account_control: UserAccountControl,
    #[ntds_attribute("ATTq589876")] ds_last_logon: WindowsFileTime,
    #[ntds_attribute("ATTq591520")] ds_last_logon_time_stamp: WindowsFileTime,
    #[ntds_attribute("ATTq589983")] ds_account_expires: WindowsFileTime,
    #[ntds_attribute("ATTq589920")] ds_password_last_set: WindowsFileTime,
    #[ntds_attribute("ATTq589873")] ds_bad_pwd_time: WindowsFileTime,
    #[ntds_attribute("ATTj589993")] ds_logon_count: i32,
    #[ntds_attribute("ATTj589836")] ds_bad_pwd_count: i32,
    #[ntds_attribute("ATTj589922")] ds_primary_group_id: i32,
    #[ntds_attribute("ATTk36")]     ds_aduser_objects: String,
    #[ntds_attribute("ATTm13")]     ds_att_comment: String,
    #[ntds_attribute("ATTm590443")] ds_dns_host_name: String,
    #[ntds_attribute("ATTm590187")] ds_os_name: String,
    #[ntds_attribute("ATTm590188")] ds_os_version: String,
    #[ntds_attribute("ATTj131122")] ds_link_id: u32,
    #[ntds_attribute("ATTm131532")] ds_ldap_display_name: String,
    #[ntds_attribute("ATTr591234")] ds_creator_sid: Sid,
    #[ntds_attribute("ATTj589974")] ds_admin_count: i32,
    #[ntds_attribute("ATTi131120")] ds_is_deleted: bool,

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

macro_rules! ntds_attribute {
    ($cls: ident, $id: expr) => {$cls::load_by_internal_id(&mut temporary_mapping, $id)?,};
}

impl ColumnInfoMapping {
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
            ds_record_id: ntds_attribute!(Self, "DNT_col"),
            ds_parent_record_id: ntds_attribute!("PDNT_col"),
            ds_record_time: ntds_attribute!("time_col"),
            ds_ancestors: ntds_attribute!("PDNT_col"),
            ds_object_type_id: ntds_attribute!("PDNT_col"),
            ds_object_name: ntds_attribute!("PDNT_col"),
            ds_object_name2: ntds_attribute!("PDNT_col"),
            ds_when_created: ntds_attribute!("PDNT_col"),
            ds_when_changed: ntds_attribute!("PDNT_col"),
            ds_sid: ntds_attribute!("PDNT_col"),
            ds_sam_account_name: ntds_attribute!("PDNT_col"),
            ds_user_principal_name: ntds_attribute!("PDNT_col"),
            ds_sam_account_type: ntds_attribute!("PDNT_col"),,
            ds_user_account_control: ntds_attribute!("PDNT_col"),
            ds_last_logon: ntds_attribute!("PDNT_col"),
            ds_last_logon_time_stamp: ntds_attribute!("PDNT_col"),
            ds_account_expires: ntds_attribute!("PDNT_col"),
            ds_password_last_set: ntds_attribute!("PDNT_col"),
            ds_bad_pwd_time: ntds_attribute!("PDNT_col"),
            ds_logon_count: ntds_attribute!("PDNT_col"),
            ds_bad_pwd_count:ntds_attribute!("PDNT_col"),
            ds_primary_group_id: ntds_attribute!("PDNT_col"),
            ds_aduser_objects: ntds_attribute!("PDNT_col"),
            ds_att_comment: ntds_attribute!("PDNT_col"),
            ds_dns_host_name: ntds_attribute!("PDNT_col"),
            ds_os_name: ntds_attribute!("PDNT_col"),
            ds_os_version: ntds_attribute!("PDNT_col"),
            ds_link_id: ntds_attribute!("PDNT_col"),
            ds_ldap_display_name: ntds_attribute!("PDNT_col"),
            ds_creator_sid: ntds_attribute!("PDNT_col"),
            ds_admin_count: ntds_attribute!("PDNT_col"),
            ds_is_deleted: ntds_attribute!("PDNT_col"),
        };
        Ok(mapping)
    }
    pub fn name_of_column(&self, index: &i32) -> Option<&String> {
        self.column_names.get(index)
    }

    fn load_by_internal_id<F>(mut temporary_mapping: &HashMap<&str, F>, internal_id: &str)-> Result<F> {
        temporary_mapping.remove(internal_id).expect(&format!("missing column '{internal_id}'"))
    }
}

