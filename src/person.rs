use std::collections::HashMap;

use bodyfile::Bodyfile3Line;
use serde::{Serialize, Serializer};

use crate::{dbrecord::{DbRecord, FromDbRecord}, ColumnInfoMapping, skip_all_attributes, user_account_control::UserAccountControl};
use anyhow::Result;
use chrono::{Utc, DateTime};

#[derive(Serialize)]
pub (crate) struct Person {

    #[serde(serialize_with = "to_ts")]
    record_time: Option<DateTime<Utc>>,
    
    #[serde(serialize_with = "to_ts")]
    when_created: Option<DateTime<Utc>>,

    #[serde(serialize_with = "to_ts")]
    when_changed: Option<DateTime<Utc>>,

    sid: Option<String>,
    sam_account_name: Option<String>,
    user_principal_name: Option<String>,
    samaccount_type: Option<i32>,
    user_account_control: Option<UserAccountControl>,

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
    comment: Option<String>,
    #[serde(skip)]
    nthash: Option<String>,
    #[serde(skip)]
    lmhash: Option<String>,
    #[serde(skip)]
    nthash_history: Option<String>,
    #[serde(skip)]
    lmhash_history: Option<String>,
    unix_password: Option<String>,
    aduser_objects: Option<String>,
    #[serde(skip)]
    supplemental_credentials: Option<String>,

    #[serde(skip_serializing_if = "skip_all_attributes")]
    all_attributes: HashMap<String, String>,
}

fn to_ts<S>(ts: &Option<DateTime<Utc>>, s: S) -> Result<S::Ok, S::Error> where S: Serializer {
    match ts {
        Some(ts) => s.serialize_str(&ts.to_rfc3339()),
        None => s.serialize_str("")
    }
}

impl FromDbRecord for Person {
    fn from(dbrecord: DbRecord, mapping: &ColumnInfoMapping) -> Result<Self> {
        Ok(Self {
            record_time: dbrecord.ds_record_time_index(mapping)?,
            when_created: dbrecord.ds_when_created_index(mapping)?,
            when_changed: dbrecord.ds_when_changed_index(mapping)?,
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
            comment: dbrecord.ds_att_comment(mapping)?,
            nthash: dbrecord.ds_nthash_index(mapping)?,
            lmhash: dbrecord.ds_lmhash_index(mapping)?,
            nthash_history: dbrecord.ds_nthash_history_index(mapping)?,
            lmhash_history: dbrecord.ds_lmhash_history_index(mapping)?,
            unix_password: dbrecord.ds_unix_password_index(mapping)?,
            aduser_objects: dbrecord.ds_aduser_objects_index(mapping)?,
            supplemental_credentials: dbrecord.ds_supplemental_credentials_index(mapping)?,
            all_attributes: dbrecord.all_attributes(mapping),
        })
    }
}

impl From<Person> for Vec<Bodyfile3Line> {
    fn from(person: Person) -> Self {
        let mut res = Vec::new();
        if let Some(upn) =  person.sam_account_name {

            if let Some(record_time) = person.record_time {
                res.push(
                    Bodyfile3Line::new()
                        .with_owned_name(format!("{} (Person, record creation time)", upn))
                        .with_crtime(i64::max(0,record_time.timestamp()))
                );
            }
            
            if let Some(when_created) = person.when_created {
                res.push(
                    Bodyfile3Line::new()
                        .with_owned_name(format!("{} (Person, object created)", upn))
                        .with_crtime(i64::max(0,when_created.timestamp()))
                );
            }

            if let Some(when_changed) = person.when_changed {
                res.push(
                    Bodyfile3Line::new()
                        .with_owned_name(format!("{} (Person, object changed)", upn))
                        .with_crtime(i64::max(0,when_changed.timestamp()))
                );
            }

            if let Some(last_logon) = person.last_logon {
                res.push(
                    Bodyfile3Line::new()
                        .with_owned_name(format!("{} (Person, last logon on this DC)", upn))
                        .with_ctime(i64::max(0,last_logon.timestamp()))
                );
            }

            if let Some(last_logon_time_stamp) = person.last_logon_time_stamp {
                res.push(
                    Bodyfile3Line::new()
                        .with_owned_name(format!("{} (Person, last logon on any DC)", upn))
                        .with_ctime(i64::max(0,last_logon_time_stamp.timestamp()))
                );
            }

            if let Some(bad_pwd_time) = person.bad_pwd_time {
                res.push(
                    Bodyfile3Line::new()
                        .with_owned_name(format!("{} (Person, bad pwd time)", upn))
                        .with_ctime(i64::max(0,bad_pwd_time.timestamp()))
                );
            }

            if let Some(password_last_set) = person.password_last_set {
                res.push(
                    Bodyfile3Line::new()
                        .with_owned_name(format!("{} (Person, password last set)", upn))
                        .with_ctime(i64::max(0,password_last_set.timestamp()))
                );
            }
        }
        res
    }
}