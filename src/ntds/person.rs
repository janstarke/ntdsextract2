use std::collections::HashMap;

use bodyfile::Bodyfile3Line;
use getset::Getters;
use serde::{Deserialize, Serialize};

use crate::{ntds, OutputOptions};
use crate::ntds::ObjectType;
use crate::serialization::*;
use crate::win32_types::{
    SamAccountType, Sid, TruncatedWindowsFileTime, UserAccountControl, WindowsFileTime,
};
use anyhow::{bail, Result};

use super::DataTableRecord;

#[derive(Getters, Serialize, Deserialize)]
#[getset(get = "pub")]
pub struct Person {
    sid: Option<Sid>,
    user_principal_name: Option<String>,
    sam_account_name: Option<String>,
    sam_account_type: Option<SamAccountType>,
    user_account_control: Option<UserAccountControl>,
    logon_count: Option<i32>,
    bad_pwd_count: Option<i32>,
    admin_count: Option<i32>,
    is_deleted: bool,

    //#[serde(skip_serializing)]
    #[allow(dead_code)]
    primary_group_id: Option<i32>,

    primary_group: Option<String>,

    aduser_objects: Option<String>,

    #[serde(serialize_with = "serialize_set")]
    member_of: SerializableSet,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    all_attributes: Option<HashMap<String, String>>,
}

impl ntds::Object for Person {
    fn new(
        dbrecord: DataTableRecord,
        options: &OutputOptions,
    ) -> Result<Self, anyhow::Error> {
        let object_id = dbrecord.ds_record_id()?;

        let primary_group_id = dbrecord.ds_primary_group_id()?;
        let primary_group = primary_group_id.and_then(|group_id| {
            dbrecord
                .data_table()
                .find_by_rid(group_id.try_into().unwrap())
                .and_then(|group| {
                    group
                        .ds_object_name2()
                        .expect("unable to read object name2")
                })
        });

        let member_of = if let Some(children) = dbrecord.link_table().member_of(&object_id) {
            children
                .iter()
                .filter_map(|child_id| dbrecord.data_table().find_by_id(*child_id))
                .map(|record| {
                    record
                        .ds_object_name2()
                        .expect("error while reading object name")
                        .expect("missing object name")
                })
                .collect()
        } else {
            vec![]
        };

        let all_attributes = if options.display_all_attributes() {
            Some(dbrecord.all_attributes())
        } else {
            None
        };

        Ok(Self {
            record_time: dbrecord.ds_record_time()?.unwrap(),
            when_created: dbrecord.ds_when_created()?,
            when_changed: dbrecord.ds_when_changed()?,
            sid: dbrecord.ds_sid()?,
            sam_account_name: dbrecord.ds_sam_account_name()?,
            user_principal_name: dbrecord.ds_user_principal_name()?,
            sam_account_type: dbrecord.ds_sam_account_type()?,
            user_account_control: dbrecord.ds_user_account_control()?,
            last_logon: dbrecord.ds_last_logon()?,
            last_logon_time_stamp: dbrecord.ds_last_logon_time_stamp()?,
            account_expires: dbrecord.ds_account_expires()?,
            password_last_set: dbrecord.ds_password_last_set()?,
            bad_pwd_time: dbrecord.ds_bad_pwd_time()?,
            logon_count: dbrecord.ds_logon_count()?,
            bad_pwd_count: dbrecord.ds_bad_pwd_count()?,
            admin_count: dbrecord.ds_admin_count()?,
            is_deleted: dbrecord.ds_is_deleted()?.unwrap_or(false),
            primary_group_id,
            primary_group,
            comment: dbrecord.ds_att_comment()?,
            aduser_objects: dbrecord.ds_aduser_objects()?,
            member_of,
            all_attributes: dbrecord.all_attributes(),
        })
    }
}

impl From<Person> for Vec<Bodyfile3Line> {
    fn from(obj: Person) -> Self {
        static OT: ObjectType = ObjectType::Person;
        if let Some(upn) = obj.sam_account_name {
            vec![
                obj.record_time()
                    .map(|ts| ts.cr_entry(upn, "record creation time", OT)),
                obj.when_created()
                    .map(|ts| ts.cr_entry(upn, "object created", OT)),
                obj.when_changed()
                    .map(|ts| ts.cr_entry(upn, "object changed", OT)),
                obj.last_logon()
                    .map(|ts| ts.c_entry(upn, "last logon on this DC", OT)),
                obj.last_logon_time_stamp()
                    .map(|ts| ts.c_entry(upn, "last logon on any DC", OT)),
                obj.bad_pwd_time()
                    .map(|ts| ts.c_entry(upn, "bad pwd time", OT)),
                obj.password_last_set()
                    .map(|ts| ts.c_entry(upn, "password last set", OT)),
            ]
            .iter()
            .filter_map(|x| x)
            .collect()
        } else {
            Vec::new()
        }
    }
}
