use std::collections::HashMap;

use bodyfile::Bodyfile3Line;
use serde::Serialize;

use crate::{DbRecord, FromDbRecord, constants::TYPENAME_COMPUTER, skip_all_attributes, win32_types::{UserAccountControl, SamAccountType, Sid}, data_table_ext::DataTableExt};
use anyhow::Result;
use chrono::{Utc, DateTime};
use crate::serialization::*;

#[derive(Serialize)]
pub (crate) struct Computer {


    sid: Option<Sid>,
    sam_account_name: Option<String>,
    sam_account_type: Option<SamAccountType>,
    user_account_control: Option<UserAccountControl>,

    dnshost_name: Option<String>,
    osname: Option<String>,
    osversion: Option<String>,

    logon_count: Option<i32>,
    bad_pwd_count: Option<i32>,
    primary_group_id: Option<i32>,

    comment: Option<String>,

    #[serde(serialize_with = "to_ts")]
    record_time: Option<DateTime<Utc>>,
    
    #[serde(serialize_with = "to_ts")]
    when_created: Option<DateTime<Utc>>,

    #[serde(serialize_with = "to_ts")]
    when_changed: Option<DateTime<Utc>>,

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

    #[serde(skip_serializing_if = "skip_all_attributes")]
    all_attributes: HashMap<String, String>,

/*
    DS_RECOVERY_PASSWORD_INDEX_NAME,
    DS_FVEKEY_PACKAGE_INDEX_NAME,
    DS_VOLUME_GUIDINDEX_NAME,
    DS_RECOVERY_GUIDINDEX_NAME
     */
}

impl FromDbRecord for Computer {
    fn from(dbrecord: DbRecord, data_table: &DataTableExt) -> Result<Self> {
        let mapping = data_table.mapping();
        Ok(Self {
            record_time: dbrecord.ds_record_time(mapping)?,
            when_created: dbrecord.ds_when_created(mapping)?,
            when_changed: dbrecord.ds_when_changed(mapping)?,
            sid: dbrecord.ds_sid(mapping)?,
            sam_account_name: dbrecord.ds_sam_account_name(mapping)?,
            sam_account_type: dbrecord.ds_sam_account_type(mapping)?,
            user_account_control: dbrecord.ds_user_account_control(mapping)?,
            last_logon: dbrecord.ds_last_logon(mapping)?,
            last_logon_time_stamp: dbrecord.ds_last_logon_time_stamp(mapping)?,
            account_expires: dbrecord.ds_account_expires(mapping)?,
            password_last_set: dbrecord.ds_password_last_set(mapping)?,
            bad_pwd_time: dbrecord.ds_bad_pwd_time(mapping)?,
            logon_count: dbrecord.ds_logon_count(mapping)?,
            bad_pwd_count: dbrecord.ds_bad_pwd_count(mapping)?,
            primary_group_id: dbrecord.ds_primary_group_id(mapping)?,
            dnshost_name: dbrecord.ds_dns_host_name(mapping)?,
            osname: dbrecord.ds_os_name(mapping)?,
            osversion: dbrecord.ds_os_version(mapping)?,
            comment: dbrecord.ds_att_comment(mapping)?,
            all_attributes: dbrecord.all_attributes(mapping),
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