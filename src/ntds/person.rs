use std::collections::HashMap;
use std::convert::identity;

use bodyfile::Bodyfile3Line;
use getset::Getters;
use serde::{Deserialize, Serialize};

use crate::ntds::ObjectType;
use crate::win32_types::{
    SamAccountType, Sid, TruncatedWindowsFileTime, UserAccountControl, WindowsFileTime,
};
use crate::{ntds, OutputOptions};
use crate::{serialization::*, RecordHasId, RecordHasRid};
use anyhow::Result;

use super::{DataTable, DataTableRecord, LinkTable};
use crate::win32_types::TimelineEntry;

#[derive(Getters, Serialize)]
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

    //aduser_objects: Option<String>,
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
        data_table: &DataTable,
        link_table: &LinkTable,
    ) -> Result<Self, anyhow::Error> {
        let object_id = dbrecord.ds_record_id()?;

        let primary_group_id = dbrecord.att_primary_group_id_opt()?;
        let primary_group = primary_group_id.and_then(|group_id| {
            data_table
                .data_table()
                .find_p(RecordHasRid(group_id.try_into().unwrap()))
                .map(|group| group.att_object_name2().unwrap())
        });

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

        let member_of = if *options.flat_serialization() {
            SerializableSet::Flat(member_of)
        } else {
            SerializableSet::Complex(member_of)
        };

        let all_attributes = if *options.display_all_attributes() {
            Some(dbrecord.all_attributes())
        } else {
            None
        };

        Ok(Self {
            record_time: dbrecord.ds_record_time_opt()?,
            when_created: dbrecord.att_when_created_opt()?,
            when_changed: dbrecord.att_when_changed_opt()?,
            sid: dbrecord.att_object_sid_opt()?,
            sam_account_name: dbrecord.att_sam_account_name_opt()?,
            user_principal_name: dbrecord.att_user_principal_name_opt()?,
            sam_account_type: dbrecord.att_sam_account_type_opt()?,
            user_account_control: dbrecord.att_user_account_control_opt()?,
            last_logon: dbrecord.att_last_logon_opt()?,
            last_logon_time_stamp: dbrecord.att_last_logon_time_stamp_opt()?,
            account_expires: dbrecord.att_account_expires_opt()?,
            password_last_set: dbrecord.att_password_last_set_opt()?,
            bad_pwd_time: dbrecord.att_bad_pwd_time_opt()?,
            logon_count: dbrecord.att_logon_count_opt()?,
            bad_pwd_count: dbrecord.att_bad_pwd_count_opt()?,
            admin_count: dbrecord.att_admin_count_opt()?,
            is_deleted: dbrecord.att_is_deleted_opt()?.unwrap_or(false),
            primary_group_id,
            primary_group,
            comment: dbrecord.att_comment_opt()?,
            //aduser_objects: dbrecord.att_u()?,
            member_of,
            all_attributes,
        })
    }
}

impl From<Person> for Vec<Bodyfile3Line> {
    fn from(obj: Person) -> Self {
        static OT: ObjectType = ObjectType::Person;
        if let Some(upn) = obj.sam_account_name {
            vec![
                obj.record_time()
                    .map(|ts| ts.cr_entry(&upn, "record creation time", OT)),
                obj.when_created()
                    .map(|ts| ts.cr_entry(&upn, "object created", OT)),
                obj.when_changed()
                    .map(|ts| ts.cr_entry(&upn, "object changed", OT)),
                obj.last_logon()
                    .map(|ts| ts.c_entry(&upn, "last logon on this DC", OT)),
                obj.last_logon_time_stamp()
                    .map(|ts| ts.c_entry(&upn, "last logon on any DC", OT)),
                obj.bad_pwd_time()
                    .map(|ts| ts.c_entry(&upn, "bad pwd time", OT)),
                obj.password_last_set()
                    .map(|ts| ts.c_entry(&upn, "password last set", OT)),
            ]
            .into_iter()
            .filter_map(identity)
            .collect()
        } else {
            Vec::new()
        }
    }
}
