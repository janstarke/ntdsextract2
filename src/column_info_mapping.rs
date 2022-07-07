use std::collections::HashMap;
use libesedb::Table;
use crate::column_information::ColumnInformation;
use crate::constants::*;
use anyhow::Result;

pub (crate) struct ColumnInfoMapping {
    pub (crate) dsRecordIdIndex: ColumnInformation,
    pub (crate) dsParentRecordIdIndex: ColumnInformation,
    pub (crate) ds_record_time_index: ColumnInformation,
    pub (crate) ds_ancestors_index: ColumnInformation,
    pub (crate) dsObjectTypeIdIndex: ColumnInformation,
    pub (crate) dsObjectNameIndex: ColumnInformation,
    pub (crate) dsObjectName2Index: ColumnInformation,
    pub (crate) dsObjectGUIDIndex: ColumnInformation,
    pub (crate) ds_when_created_index: ColumnInformation,
    pub (crate) ds_when_changed_index: ColumnInformation,
    pub (crate) dsUSNCreatedIndex: ColumnInformation,
    pub (crate) dsUSNChangedIndex: ColumnInformation,
    pub (crate) dsObjectColIndex: ColumnInformation,
    pub (crate) dsIsDeletedIndex: ColumnInformation,

    pub (crate) dsOrigContainerIdIndex: ColumnInformation,

    pub (crate) ds_sidindex: ColumnInformation,
    pub (crate) ds_samaccount_name_index: ColumnInformation,
    pub (crate) ds_user_principal_name_index: ColumnInformation,
    pub (crate) ds_samaccount_type_index: ColumnInformation,
    pub (crate) ds_user_account_control_index: ColumnInformation,
    pub (crate) ds_last_logon_index: ColumnInformation,
    pub (crate) ds_last_logon_time_stamp_index: ColumnInformation,
    pub (crate) ds_account_expires_index: ColumnInformation,
    pub (crate) ds_password_last_set_index: ColumnInformation,
    pub (crate) ds_bad_pwd_time_index: ColumnInformation,
    pub (crate) ds_logon_count_index: ColumnInformation,
    pub (crate) ds_bad_pwd_count_index: ColumnInformation,
    pub (crate) ds_primary_group_id_index: ColumnInformation,
    pub (crate) ds_nthash_index: ColumnInformation,
    pub (crate) ds_lmhash_index: ColumnInformation,
    pub (crate) ds_nthash_history_index: ColumnInformation,
    pub (crate) ds_lmhash_history_index: ColumnInformation,
    pub (crate) ds_unix_password_index: ColumnInformation,
    pub (crate) ds_aduser_objects_index: ColumnInformation,
    pub (crate) ds_supplemental_credentials_index: ColumnInformation,

    pub (crate) dnshost_name: ColumnInformation,
    pub (crate) osname: ColumnInformation,
    pub (crate) osversion: ColumnInformation,
    pub (crate) dsRecoveryPasswordIndex: ColumnInformation,
    pub (crate) dsFVEKeyPackageIndex: ColumnInformation,
    pub (crate) dsVolumeGUIDIndex: ColumnInformation,
    pub (crate) dsRecoveryGUIDIndex: ColumnInformation,
    pub (crate) dsDialInAccessPermission: ColumnInformation,
    pub (crate) dsPEKIndex: ColumnInformation,
}

impl ColumnInfoMapping {
    pub fn from(data_table: &Table) -> Result<Self> {
        let mut temporary_mapping = HashMap::new();
        for index in 0..data_table.count_columns()? {
            let column_res = data_table.column(index)?;
            let col_info = ColumnInformation::new(
                index,
                column_res.name()?,
                column_res.variant()?
            );
            temporary_mapping.insert(column_res.name()?, col_info);
        }

        let mapping = ColumnInfoMapping {
            dsRecordIdIndex: temporary_mapping.remove(DS_RECORD_ID_INDEX_NAME).unwrap(),
            dsParentRecordIdIndex: temporary_mapping.remove(DS_PARENT_RECORD_ID_INDEX_NAME).unwrap(),
            ds_record_time_index: temporary_mapping.remove(DS_RECORD_TIME_INDEX_NAME).unwrap(),
            ds_ancestors_index: temporary_mapping.remove(DS_ANCESTORS_INDEX_NAME).unwrap(),
            dsObjectTypeIdIndex: temporary_mapping.remove(DS_OBJECT_TYPE_ID_INDEX_NAME).unwrap(),
            dsObjectNameIndex: temporary_mapping.remove(DS_OBJECT_NAME_INDEX_NAME).unwrap(),
            dsObjectName2Index: temporary_mapping.remove(DS_OBJECT_NAME2_INDEX_NAME).unwrap(),
            dsObjectGUIDIndex: temporary_mapping.remove(DS_OBJECT_GUIDINDEX_NAME).unwrap(),
            ds_when_created_index: temporary_mapping.remove(DS_WHEN_CREATED_INDEX_NAME).unwrap(),
            ds_when_changed_index: temporary_mapping.remove(DS_WHEN_CHANGED_INDEX_NAME).unwrap(),
            dsUSNCreatedIndex: temporary_mapping.remove(DS_USNCREATED_INDEX_NAME).unwrap(),
            dsUSNChangedIndex: temporary_mapping.remove(DS_USNCHANGED_INDEX_NAME).unwrap(),
            dsObjectColIndex: temporary_mapping.remove(DS_OBJECT_COL_INDEX_NAME).unwrap(),
            dsIsDeletedIndex: temporary_mapping.remove(DS_IS_DELETED_INDEX_NAME).unwrap(),
            dsOrigContainerIdIndex: temporary_mapping.remove(DS_ORIG_CONTAINER_ID_INDEX_NAME).unwrap(),
            ds_sidindex: temporary_mapping.remove(DS_SIDINDEX_NAME).unwrap(),
            ds_samaccount_name_index: temporary_mapping.remove(DS_SAMACCOUNT_NAME_INDEX_NAME).unwrap(),
            ds_user_principal_name_index: temporary_mapping.remove(DS_USER_PRINCIPAL_NAME_INDEX_NAME).unwrap(),
            ds_samaccount_type_index: temporary_mapping.remove(DS_SAMACCOUNT_TYPE_INDEX_NAME).unwrap(),
            ds_user_account_control_index: temporary_mapping.remove(DS_USER_ACCOUNT_CONTROL_INDEX_NAME).unwrap(),
            ds_last_logon_index: temporary_mapping.remove(DS_LAST_LOGON_INDEX_NAME).unwrap(),
            ds_last_logon_time_stamp_index: temporary_mapping.remove(DS_LAST_LOGON_TIME_STAMP_INDEX_NAME).unwrap(),
            ds_account_expires_index: temporary_mapping.remove(DS_ACCOUNT_EXPIRES_INDEX_NAME).unwrap(),
            ds_password_last_set_index: temporary_mapping.remove(DS_PASSWORD_LAST_SET_INDEX_NAME).unwrap(),
            ds_bad_pwd_time_index: temporary_mapping.remove(DS_BAD_PWD_TIME_INDEX_NAME).unwrap(),
            ds_logon_count_index: temporary_mapping.remove(DS_LOGON_COUNT_INDEX_NAME).unwrap(),
            ds_bad_pwd_count_index: temporary_mapping.remove(DS_BAD_PWD_COUNT_INDEX_NAME).unwrap(),
            ds_primary_group_id_index: temporary_mapping.remove(DS_PRIMARY_GROUP_ID_INDEX_NAME).unwrap(),
            ds_nthash_index: temporary_mapping.remove(DS_NTHASH_INDEX_NAME).unwrap(),
            ds_lmhash_index: temporary_mapping.remove(DS_LMHASH_INDEX_NAME).unwrap(),
            ds_nthash_history_index: temporary_mapping.remove(DS_NTHASH_HISTORY_INDEX_NAME).unwrap(),
            ds_lmhash_history_index: temporary_mapping.remove(DS_LMHASH_HISTORY_INDEX_NAME).unwrap(),
            ds_unix_password_index: temporary_mapping.remove(DS_UNIX_PASSWORD_INDEX_NAME).unwrap(),
            ds_aduser_objects_index: temporary_mapping.remove(DS_ADUSER_OBJECTS_INDEX_NAME).unwrap(),
            ds_supplemental_credentials_index: temporary_mapping.remove(DS_SUPPLEMENTAL_CREDENTIALS_INDEX_NAME).unwrap(),
            dnshost_name: temporary_mapping.remove(DS_DNSHOST_NAME_INDEX_NAME).unwrap(),
            osname: temporary_mapping.remove(DS_OSNAME_INDEX_NAME).unwrap(),
            osversion: temporary_mapping.remove(DS_OSVERSION_INDEX_NAME).unwrap(),
            dsRecoveryPasswordIndex: temporary_mapping.remove(DS_RECOVERY_PASSWORD_INDEX_NAME).unwrap(),
            dsFVEKeyPackageIndex: temporary_mapping.remove(DS_FVEKEY_PACKAGE_INDEX_NAME).unwrap(),
            dsVolumeGUIDIndex: temporary_mapping.remove(DS_VOLUME_GUIDINDEX_NAME).unwrap(),
            dsRecoveryGUIDIndex: temporary_mapping.remove(DS_RECOVERY_GUIDINDEX_NAME).unwrap(),
            dsDialInAccessPermission: temporary_mapping.remove(DS_DIAL_IN_ACCESS_PERMISSION_NAME).unwrap(),
            dsPEKIndex: temporary_mapping.remove(DS_PEKINDEX_NAME).unwrap(),
        };
        
        Ok(mapping)
    }
}