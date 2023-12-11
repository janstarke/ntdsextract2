use std::collections::HashMap;

use bodyfile::Bodyfile3Line;
use getset::Getters;
use serde::{Serialize, Deserialize};

use crate::{
    ntds,
    win32_types::{
        SamAccountType, Sid, TimelineEntry, TruncatedWindowsFileTime, UserAccountControl,
        WindowsFileTime,
    },
    OutputOptions, RecordHasId, serialization::{SerializationType, StringSet},
};
use anyhow::Result;

use super::{DataTableRecord, ObjectType};

#[derive(Serialize, Getters, Deserialize)]
#[getset(get = "pub")]
#[serde(bound = "T: SerializationType")]
pub struct Computer<T: SerializationType> {
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
    is_deleted: bool,

    member_of: StringSet<T>,

    comment: Option<String>,

    record_time: Option<TruncatedWindowsFileTime>,
    when_created: Option<TruncatedWindowsFileTime>,
    when_changed: Option<TruncatedWindowsFileTime>,
    last_logon: Option<WindowsFileTime>,
    last_logon_time_stamp: Option<WindowsFileTime>,
    account_expires: Option<WindowsFileTime>,
    password_last_set: Option<WindowsFileTime>,
    bad_pwd_time: Option<WindowsFileTime>,

    created_sid: Option<Sid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    all_attributes: Option<HashMap<String, String>>,
}

impl<T> ntds::Object for Computer<T> where T: SerializationType {
    fn new(
        dbrecord: DataTableRecord,
        options: &OutputOptions,
        data_table: &ntds::DataTable,
        link_table: &ntds::LinkTable,
    ) -> Result<Self, anyhow::Error> {
        let object_id = dbrecord.ds_record_id()?;

        let member_of = if let Some(children) = link_table.member_of(&object_id) {
            children
                .iter()
                .filter_map(|child_id| data_table.data_table().find_p(RecordHasId(*child_id)))
                .map(|record| {
                    record
                        .att_object_name2()
                        .expect("error while reading object name")
                })
                .collect()
        } else {
            vec![]
        };

        let all_attributes = if *options.display_all_attributes() {
            Some(dbrecord.all_attributes())
        } else {
            None
        };

        Ok(Self {
            record_time: dbrecord.ds_record_time().ok(),
            when_created: dbrecord.att_when_created().ok(),
            when_changed: dbrecord.att_when_changed().ok(),
            sid: dbrecord.att_object_sid().ok(),
            sam_account_name: dbrecord.att_sam_account_name().ok(),
            sam_account_type: dbrecord.att_sam_account_type().ok(),
            user_account_control: dbrecord.att_user_account_control().ok(),
            last_logon: dbrecord.att_last_logon().ok(),
            last_logon_time_stamp: dbrecord.att_last_logon_time_stamp().ok(),
            account_expires: dbrecord.att_account_expires().ok(),
            password_last_set: dbrecord.att_password_last_set().ok(),
            bad_pwd_time: dbrecord.att_bad_pwd_time().ok(),
            logon_count: dbrecord.att_logon_count().ok(),
            bad_pwd_count: dbrecord.att_bad_pwd_count().ok(),
            primary_group_id: dbrecord.att_primary_group_id().ok(),
            dnshost_name: dbrecord.att_dns_host_name().ok(),
            osname: dbrecord.att_os_name().ok(),
            osversion: dbrecord.att_os_version().ok(),
            comment: dbrecord.att_comment().ok(),
            created_sid: dbrecord.att_creator_sid().ok(),
            is_deleted: dbrecord.att_is_deleted().unwrap_or(false),
            all_attributes,
            member_of: member_of.into(),
        })
    }
}

impl<T> From<Computer<T>> for Vec<Bodyfile3Line> where T: SerializationType {
    fn from(obj: Computer<T>) -> Self {
        static OT: ObjectType = ObjectType::Computer;
        if let Some(upn) = &obj.sam_account_name {
            vec![
                obj.record_time()
                    .as_ref()
                    .map(|ts| ts.cr_entry(upn, "record creation time", OT)),
                obj.when_created()
                    .as_ref()
                    .map(|ts| ts.cr_entry(upn, "object created", OT)),
                obj.when_changed()
                    .as_ref()
                    .map(|ts| ts.cr_entry(upn, "object changed", OT)),
                obj.last_logon()
                    .as_ref()
                    .map(|ts| ts.c_entry(upn, "last logon on this DC", OT)),
                obj.last_logon_time_stamp()
                    .as_ref()
                    .map(|ts| ts.c_entry(upn, "last logon on any DC", OT)),
                obj.bad_pwd_time()
                    .as_ref()
                    .map(|ts| ts.c_entry(upn, "bad pwd time", OT)),
                obj.password_last_set()
                    .as_ref()
                    .map(|ts| ts.c_entry(upn, "password last set", OT)),
            ]
            .into_iter()
            .flatten()
            .collect()
        } else {
            Vec::new()
        }
    }
}
