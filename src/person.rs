use serde::{Serialize, Serializer};

use crate::{dbrecord::DbRecord, ColumnInfoMapping};
use anyhow::Result;
use chrono::{Utc, DateTime};

#[derive(Serialize)]
pub (crate) struct Person {
    sid: Option<String>,
    sam_account_name: Option<String>,
    user_principal_name: Option<String>,
    samaccount_type: Option<i32>,
    user_account_control: Option<i32>,

    #[serde(serialize_with = "to_ts")]
    last_logon: Option<DateTime<Utc>>,

    #[serde(serialize_with = "to_ts")]
    last_logon_time_stamp: Option<DateTime<Utc>>,

    #[serde(serialize_with = "to_ts")]
    account_expires: Option<DateTime<Utc>>,
    
    #[serde(serialize_with = "to_ts")]
    password_last_set: Option<DateTime<Utc>>,

    #[serde(serialize_with = "to_ts")]
    bad_pwd_time: Option<DateTime<Utc>>,
    logon_count: Option<i32>,
    bad_pwd_count: Option<i32>,
    primary_group_id: Option<i32>,
    nthash: Option<String>,
    lmhash: Option<String>,
    nthash_history: Option<String>,
    lmhash_history: Option<String>,
    unix_password: Option<String>,
    aduser_objects: Option<String>,
    supplemental_credentials: Option<String>,
}

fn to_ts<S>(ts: &Option<DateTime<Utc>>, s: S) -> Result<S::Ok, S::Error> where S: Serializer {
    match ts {
        Some(ts) => s.serialize_str(&ts.to_rfc3339()),
        None => s.serialize_str("")
    }
}

impl Person {
    pub fn from(dbrecord: DbRecord, mapping: &ColumnInfoMapping) -> Result<Self> {
        Ok(Self {
            sid: dbrecord.ds_sidindex(mapping)?,
            sam_account_name: dbrecord.ds_samaccount_name_index(mapping)?,
            user_principal_name: dbrecord.ds_user_principal_name_index(mapping)?,
            samaccount_type: dbrecord.ds_samaccount_type_index(mapping)?,
            user_account_control: dbrecord.ds_user_account_control_index(mapping)?,
            last_logon: dbrecord.ds_last_logon_index(mapping)?,
            last_logon_time_stamp: dbrecord.ds_last_logon_time_stamp_index(mapping)?,
            account_expires: dbrecord.ds_account_expires_index(mapping)?,
            password_last_set: dbrecord.ds_password_last_set_index(mapping)?,
            bad_pwd_time: dbrecord.ds_bad_pwd_time_index(mapping)?,
            logon_count: dbrecord.ds_logon_count_index(mapping)?,
            bad_pwd_count: dbrecord.ds_bad_pwd_count_index(mapping)?,
            primary_group_id: dbrecord.ds_primary_group_id_index(mapping)?,
            nthash: dbrecord.ds_nthash_index(mapping)?,
            lmhash: dbrecord.ds_lmhash_index(mapping)?,
            nthash_history: dbrecord.ds_nthash_history_index(mapping)?,
            lmhash_history: dbrecord.ds_lmhash_history_index(mapping)?,
            unix_password: dbrecord.ds_unix_password_index(mapping)?,
            aduser_objects: dbrecord.ds_aduser_objects_index(mapping)?,
            supplemental_credentials: dbrecord.ds_supplemental_credentials_index(mapping)?,
        })
    }
}