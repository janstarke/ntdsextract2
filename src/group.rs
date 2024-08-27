use std::collections::HashMap;

use bodyfile::Bodyfile3Line;
use serde::Serialize;

use crate::column_info_mapping::{DbRecord, FromDbRecord};
use crate::{
    skip_all_attributes,
    win32_types::{
        SamAccountType, Sid, TruncatedWindowsFileTime, UserAccountControl, WindowsFileTime,
    },
};
use crate::{serde_flat_serialization, serialization::*, CDatabase};
use anyhow::{bail, Result};

#[derive(Serialize)]
pub(crate) struct Group {
    sid: Option<Sid>,
    user_principal_name: Option<String>,
    sam_account_name: Option<String>,
    sam_account_type: Option<SamAccountType>,
    user_account_control: Option<UserAccountControl>,

    #[serde(
        skip_serializing_if = "serde_flat_serialization",
        serialize_with = "serialize_object_list"
    )]
    members: Vec<String>,
    logon_count: Option<i32>,
    bad_pwd_count: Option<i32>,
    primary_group_id: Option<i32>,
    aduser_objects: Option<String>,

    #[serde(serialize_with = "serialize_object_list")]
    member_of: Vec<String>,

    comment: Option<String>,

    #[serde(serialize_with = "to_ts")]
    record_time: Option<TruncatedWindowsFileTime>,

    #[serde(serialize_with = "to_ts")]
    when_created: Option<TruncatedWindowsFileTime>,

    #[serde(serialize_with = "to_ts")]
    when_changed: Option<TruncatedWindowsFileTime>,

    #[serde(serialize_with = "to_ts")]
    last_logon: Option<WindowsFileTime>,

    #[serde(serialize_with = "to_ts")]
    last_logon_time_stamp: Option<WindowsFileTime>,

    #[serde(serialize_with = "to_ts")]
    account_expires: Option<WindowsFileTime>,

    #[serde(serialize_with = "to_ts")]
    password_last_set: Option<WindowsFileTime>,

    #[serde(serialize_with = "to_ts")]
    bad_pwd_time: Option<WindowsFileTime>,

    #[serde(skip_serializing_if = "skip_all_attributes")]
    all_attributes: HashMap<String, String>,
}

impl FromDbRecord for Group {
    fn from(dbrecord: &DbRecord, database: &CDatabase) -> Result<Self> {
        let data_table = database.data_table();

        let mapping = data_table.mapping();
        let object_id = match dbrecord.ds_record_id(mapping)? {
            Some(id) => id,
            None => bail!("object has no record id"),
        };
        let members = if let Some(children) = database.link_table().members(&object_id) {
            children
                .iter()
                .filter_map(|child_id| {
                    data_table
                        .data_table()
                        .find_by_id(data_table.mapping(), *child_id)
                })
                .map(|record| {
                    record
                        .ds_object_name2(mapping)
                        .expect("error while reading object name")
                        .expect("missing object name")
                })
                .collect()
        } else {
            vec![]
        };

        let member_of = if let Some(children) = database.link_table().member_of(&object_id) {
            children
                .iter()
                .filter_map(|child_id| {
                    data_table
                        .data_table()
                        .find_by_id(data_table.mapping(), *child_id)
                })
                .map(|record| {
                    record
                        .ds_object_name2(mapping)
                        .expect("error while reading object name")
                        .expect("missing object name")
                })
                .collect()
        } else {
            vec![]
        };

        Ok(Self {
            record_time: dbrecord.ds_record_time(mapping)?,
            when_created: dbrecord.ds_when_created(mapping)?,
            when_changed: dbrecord.ds_when_changed(mapping)?,
            sid: dbrecord.ds_sid(mapping)?,
            sam_account_name: dbrecord.ds_sam_account_name(mapping)?,
            user_principal_name: dbrecord.ds_user_principal_name(mapping)?,
            sam_account_type: dbrecord.ds_sam_account_type(mapping)?,
            members,
            user_account_control: dbrecord.ds_user_account_control(mapping)?,
            last_logon: dbrecord.ds_last_logon(mapping)?,
            last_logon_time_stamp: dbrecord.ds_last_logon_time_stamp(mapping)?,
            account_expires: dbrecord.ds_account_expires(mapping)?,
            password_last_set: dbrecord.ds_password_last_set(mapping)?,
            bad_pwd_time: dbrecord.ds_bad_pwd_time(mapping)?,
            logon_count: dbrecord.ds_logon_count(mapping)?,
            bad_pwd_count: dbrecord.ds_bad_pwd_count(mapping)?,
            primary_group_id: dbrecord.ds_primary_group_id(mapping)?,
            comment: dbrecord.ds_att_comment(mapping)?,
            aduser_objects: dbrecord.ds_aduser_objects(mapping)?,
            all_attributes: dbrecord.all_attributes(mapping),
            member_of,
        })
    }
}

impl From<Group> for Vec<Bodyfile3Line> {
    fn from(person: Group) -> Self {
        let mut res = Vec::new();
        if let Some(upn) = person.sam_account_name {
            if let Some(record_time) = person.record_time {
                res.push(
                    Bodyfile3Line::new()
                        .with_owned_name(format!("{} (Person, record creation time)", upn))
                        .with_crtime(i64::max(0, record_time.timestamp())),
                );
            }

            if let Some(when_created) = person.when_created {
                res.push(
                    Bodyfile3Line::new()
                        .with_owned_name(format!("{} (Person, object created)", upn))
                        .with_crtime(i64::max(0, when_created.timestamp())),
                );
            }

            if let Some(when_changed) = person.when_changed {
                res.push(
                    Bodyfile3Line::new()
                        .with_owned_name(format!("{} (Person, object changed)", upn))
                        .with_crtime(i64::max(0, when_changed.timestamp())),
                );
            }

            if let Some(last_logon) = person.last_logon {
                res.push(
                    Bodyfile3Line::new()
                        .with_owned_name(format!("{} (Person, last logon on this DC)", upn))
                        .with_ctime(i64::max(0, last_logon.timestamp())),
                );
            }

            if let Some(last_logon_time_stamp) = person.last_logon_time_stamp {
                res.push(
                    Bodyfile3Line::new()
                        .with_owned_name(format!("{} (Person, last logon on any DC)", upn))
                        .with_ctime(i64::max(0, last_logon_time_stamp.timestamp())),
                );
            }

            if let Some(bad_pwd_time) = person.bad_pwd_time {
                res.push(
                    Bodyfile3Line::new()
                        .with_owned_name(format!("{} (Person, bad pwd time)", upn))
                        .with_ctime(i64::max(0, bad_pwd_time.timestamp())),
                );
            }

            if let Some(password_last_set) = person.password_last_set {
                res.push(
                    Bodyfile3Line::new()
                        .with_owned_name(format!("{} (Person, password last set)", upn))
                        .with_ctime(i64::max(0, password_last_set.timestamp())),
                );
            }
        }
        res
    }
}
