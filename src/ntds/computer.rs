use std::{collections::HashMap, convert::identity};

use bodyfile::Bodyfile3Line;
use getset::Getters;
use serde::Serialize;

use crate::{
    ntds,
    serialization::{serialize_set, to_ts, SerializableSet},
    win32_types::{
        SamAccountType, Sid, TimelineEntry, TruncatedWindowsFileTime, UserAccountControl,
        WindowsFileTime,
    },
    OutputOptions, RecordHasId,
};
use anyhow::Result;

use super::{DataTableRecord, ObjectType};

#[derive(Serialize, Getters)]
#[getset(get = "pub")]
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

        let member_of = if *options.flat_serialization() {
            SerializableSet::Flat(member_of)
        } else {
            SerializableSet::Complex(member_of)
        };

        Ok(Self {
            record_time: dbrecord.ds_record_time_opt()?,
            when_created: dbrecord.att_when_created_opt()?,
            when_changed: dbrecord.att_when_changed_opt()?,
            sid: dbrecord.att_object_sid_opt()?,
            sam_account_name: dbrecord.att_sam_account_name_opt()?,
            sam_account_type: dbrecord.att_sam_account_type_opt()?,
            user_account_control: dbrecord.att_user_account_control_opt()?,
            last_logon: dbrecord.att_last_logon_opt()?,
            last_logon_time_stamp: dbrecord.att_last_logon_time_stamp_opt()?,
            account_expires: dbrecord.att_account_expires_opt()?,
            password_last_set: dbrecord.att_password_last_set_opt()?,
            bad_pwd_time: dbrecord.att_bad_pwd_time_opt()?,
            logon_count: dbrecord.att_logon_count_opt()?,
            bad_pwd_count: dbrecord.att_bad_pwd_count_opt()?,
            primary_group_id: dbrecord.att_primary_group_id_opt()?,
            dnshost_name: dbrecord.att_dns_host_name_opt()?,
            osname: dbrecord.att_os_name_opt()?,
            osversion: dbrecord.att_os_version_opt()?,
            comment: dbrecord.att_comment_opt()?,
            created_sid: dbrecord.att_creator_sid_opt()?,
            all_attributes,
            member_of,
        })
    }
}

impl From<Computer> for Vec<Bodyfile3Line> {
    fn from(obj: Computer) -> Self {
        static OT: ObjectType = ObjectType::Computer;
        if let Some(upn) = &obj.sam_account_name {
            vec![
                obj.record_time().as_ref()
                    .map(|ts| ts.cr_entry(upn, "record creation time", OT)),
                obj.when_created().as_ref()
                    .map(|ts| ts.cr_entry(upn, "object created", OT)),
                obj.when_changed().as_ref()
                    .map(|ts| ts.cr_entry(upn, "object changed", OT)),
                obj.last_logon().as_ref()
                    .map(|ts| ts.c_entry(upn, "last logon on this DC", OT)),
                obj.last_logon_time_stamp().as_ref()
                    .map(|ts| ts.c_entry(upn, "last logon on any DC", OT)),
                obj.bad_pwd_time().as_ref()
                    .map(|ts| ts.c_entry(upn, "bad pwd time", OT)),
                obj.password_last_set().as_ref()
                    .map(|ts| ts.c_entry(upn, "password last set", OT)),
            ]
            .into_iter()
            .filter_map(identity)
            .collect()
        } else {
            Vec::new()
        }
    }
}
