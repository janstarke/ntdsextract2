use std::collections::HashMap;

use bodyfile::Bodyfile3Line;
use serde::Serialize;

use crate::{
    ntds,
    serialization::{to_ts, SerializableSet, serialize_set},
    win32_types::{
        SamAccountType, Sid, TruncatedWindowsFileTime, UserAccountControl, WindowsFileTime, TimelineEntry
    }, OutputOptions,
};
use anyhow::{bail, Result};

use super::{DataTableRecord, ObjectType};

#[derive(Serialize)]
pub struct Computer {
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

    created_sid: Option<Sid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    all_attributes: Option<HashMap<String, String>>,
}

impl ntds::Object for Computer {
    fn new(
        dbrecord: DataTableRecord,
        options: &OutputOptions,
    ) -> Result<Self, anyhow::Error> {
        let object_id = dbrecord.ds_record_id()?;

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
            sam_account_type: dbrecord.ds_sam_account_type()?,
            user_account_control: dbrecord.ds_user_account_control()?,
            last_logon: dbrecord.ds_last_logon()?,
            last_logon_time_stamp: dbrecord.ds_last_logon_time_stamp()?,
            account_expires: dbrecord.ds_account_expires()?,
            password_last_set: dbrecord.ds_password_last_set()?,
            bad_pwd_time: dbrecord.ds_bad_pwd_time()?,
            logon_count: dbrecord.ds_logon_count()?,
            bad_pwd_count: dbrecord.ds_bad_pwd_count()?,
            primary_group_id: dbrecord.ds_primary_group_id()?,
            dnshost_name: dbrecord.ds_dns_host_name()?,
            osname: dbrecord.ds_os_name()?,
            osversion: dbrecord.ds_os_version()?,
            comment: dbrecord.ds_att_comment()?,
            created_sid: dbrecord.ds_creator_sid()?,
            all_attributes,
            member_of,
        })
    }
}

impl From<Computer> for Vec<Bodyfile3Line> {
    fn from(obj: Computer) -> Self {
        static OT: ObjectType = ObjectType::Computer;
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
