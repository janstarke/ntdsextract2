use bodyfile::Bodyfile3Line;
use serde::{Serialize, Serializer};

use crate::{dbrecord::{DbRecord, FromDbRecord}, ColumnInfoMapping, constants::TYPENAME_COMPUTER};
use anyhow::Result;
use chrono::{Utc, DateTime};

#[derive(Serialize)]
pub (crate) struct Computer {

    #[serde(serialize_with = "to_ts")]
    record_time: Option<DateTime<Utc>>,
    
    #[serde(serialize_with = "to_ts")]
    when_created: Option<DateTime<Utc>>,

    #[serde(serialize_with = "to_ts")]
    when_changed: Option<DateTime<Utc>>,

    sid: Option<String>,
    sam_account_name: Option<String>,
    samaccount_type: Option<i32>,
    user_account_control: Option<i32>,

    dnshost_name: Option<String>,
    osname: Option<String>,
    osversion: Option<String>,

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

/*
    DS_RECOVERY_PASSWORD_INDEX_NAME,
    DS_FVEKEY_PACKAGE_INDEX_NAME,
    DS_VOLUME_GUIDINDEX_NAME,
    DS_RECOVERY_GUIDINDEX_NAME
     */
}

fn to_ts<S>(ts: &Option<DateTime<Utc>>, s: S) -> Result<S::Ok, S::Error> where S: Serializer {
    match ts {
        Some(ts) => s.serialize_str(&ts.to_rfc3339()),
        None => s.serialize_str("")
    }
}

impl FromDbRecord for Computer {
    fn from(dbrecord: DbRecord, mapping: &ColumnInfoMapping) -> Result<Self> {
        Ok(Self {
            record_time: dbrecord.ds_record_time_index(mapping)?,
            when_created: dbrecord.ds_when_created_index(mapping)?,
            when_changed: dbrecord.ds_when_changed_index(mapping)?,
            sid: dbrecord.ds_sidindex(mapping)?,
            sam_account_name: dbrecord.ds_samaccount_name_index(mapping)?,
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
            dnshost_name: dbrecord.dnshost_name(mapping)?,
            osname: dbrecord.osname(mapping)?,
            osversion: dbrecord.osversion(mapping)?,
        })
    }
}

impl From<Computer> for Vec<Bodyfile3Line> {
    fn from(person: Computer) -> Self {
        let mut res = Vec::new();
        if let Some(upn) =  person.sam_account_name {

            if let Some(record_time) = person.record_time {
                res.push(
                    Bodyfile3Line::new()
                        .with_owned_name(format!("{} ({}, record creation time)", upn, TYPENAME_COMPUTER))
                        .with_crtime(i64::max(0,record_time.timestamp()))
                );
            }
            
            if let Some(when_created) = person.when_created {
                res.push(
                    Bodyfile3Line::new()
                        .with_owned_name(format!("{} ({}, object created)", upn, TYPENAME_COMPUTER))
                        .with_crtime(i64::max(0,when_created.timestamp()))
                );
            }

            if let Some(when_changed) = person.when_changed {
                res.push(
                    Bodyfile3Line::new()
                        .with_owned_name(format!("{} ({}, object changed)", upn, TYPENAME_COMPUTER))
                        .with_crtime(i64::max(0,when_changed.timestamp()))
                );
            }

            if let Some(last_logon) = person.last_logon {
                res.push(
                    Bodyfile3Line::new()
                        .with_owned_name(format!("{} ({}, last logon on this DC)", upn, TYPENAME_COMPUTER))
                        .with_ctime(i64::max(0,last_logon.timestamp()))
                );
            }

            if let Some(last_logon_time_stamp) = person.last_logon_time_stamp {
                res.push(
                    Bodyfile3Line::new()
                        .with_owned_name(format!("{} ({}, last logon on any DC)", upn, TYPENAME_COMPUTER))
                        .with_ctime(i64::max(0,last_logon_time_stamp.timestamp()))
                );
            }

            if let Some(bad_pwd_time) = person.bad_pwd_time {
                res.push(
                    Bodyfile3Line::new()
                        .with_owned_name(format!("{} ({}, bad pwd time)", upn, TYPENAME_COMPUTER))
                        .with_ctime(i64::max(0,bad_pwd_time.timestamp()))
                );
            }

            if let Some(password_last_set) = person.password_last_set {
                res.push(
                    Bodyfile3Line::new()
                        .with_owned_name(format!("{} ({}, password last set)", upn, TYPENAME_COMPUTER))
                        .with_ctime(i64::max(0,password_last_set.timestamp()))
                );
            }
        }
        res
    }
}