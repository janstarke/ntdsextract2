use crate::win32_types::{Rdn, TimelineEntry, TruncatedWindowsFileTime, WindowsFileTime};
use crate::win32_types::{SamAccountType, Sid, UserAccountControl};
use crate::{OutputOptions, RdnSet, SerializationType};
use bodyfile::Bodyfile3Line;
use getset::Getters;
use serde::{Deserialize, Serialize};

use super::{DataTable, DataTableRecord, FromDataTable, HasObjectType, LinkTable};
use std::marker::PhantomData;

#[derive(Getters, Serialize, Deserialize)]
#[getset(get = "pub")]
#[serde(bound = "T: SerializationType")]
pub struct Object<T, O>
where
    O: HasObjectType,
    T: SerializationType,
{
    sid: Option<Sid>,
    user_principal_name: Option<String>,
    rdn: Option<Rdn>,
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

    primary_group: Option<Rdn>,

    //aduser_objects: Option<String>,
    member_of: RdnSet<T>,

    comment: Option<String>,

    record_time: Option<TruncatedWindowsFileTime>,

    when_created: Option<TruncatedWindowsFileTime>,
    when_changed: Option<TruncatedWindowsFileTime>,
    last_logon: Option<WindowsFileTime>,
    last_logon_time_stamp: Option<WindowsFileTime>,
    account_expires: Option<WindowsFileTime>,
    password_last_set: Option<WindowsFileTime>,
    bad_pwd_time: Option<WindowsFileTime>,

    #[serde(skip)]
    _marker: PhantomData<O>,
}

impl<T, O> FromDataTable for Object<T, O>
where
    O: HasObjectType,
    T: SerializationType,
{
    fn new(
        dbrecord: DataTableRecord,
        _options: &OutputOptions,
        data_table: &DataTable,
        link_table: &LinkTable,
    ) -> Result<Self, anyhow::Error> {
        let object_id = dbrecord.ds_record_id()?;

        let primary_group_id = dbrecord.att_primary_group_id().ok();
        let primary_group = primary_group_id.and_then(|group_id| {
            data_table
                .data_table()
                .metadata()
                .entries_with_rid(group_id.try_into().unwrap())
                .next() // there should be at most one entry with this rid
                .map(|e| {
                    data_table
                        .data_table()
                        .data_table_record_from(*e.record_ptr().esedb_row())
                        .unwrap()
                })
                .map(|group| group.att_object_name2().unwrap())
        });

        let member_of = link_table.member_names_of(object_id, data_table).into();

        Ok(Self {
            record_time: dbrecord.ds_record_time().ok(),
            when_created: dbrecord.att_when_created().ok(),
            when_changed: dbrecord.att_when_changed().ok(),
            sid: dbrecord.att_object_sid().ok(),
            sam_account_name: dbrecord.att_sam_account_name().ok(),
            rdn: dbrecord.att_object_name2().ok(),
            user_principal_name: dbrecord.att_user_principal_name().ok(),
            sam_account_type: dbrecord.att_sam_account_type().ok(),
            user_account_control: dbrecord.att_user_account_control().ok(),
            last_logon: dbrecord.att_last_logon().ok(),
            last_logon_time_stamp: dbrecord.att_last_logon_time_stamp().ok(),
            account_expires: dbrecord.att_account_expires().ok(),
            password_last_set: dbrecord.att_password_last_set().ok(),
            bad_pwd_time: dbrecord.att_bad_pwd_time().ok(),
            logon_count: dbrecord.att_logon_count().ok(),
            bad_pwd_count: dbrecord.att_bad_pwd_count().ok(),
            admin_count: dbrecord.att_admin_count().ok(),
            is_deleted: dbrecord.att_is_deleted().unwrap_or(false),
            primary_group_id,
            primary_group,
            comment: dbrecord.att_comment().ok(),
            //aduser_objects: dbrecord.att_u()?,
            member_of,
            _marker: PhantomData,
        })
    }
}

impl<T, O> From<Object<T, O>> for Vec<Bodyfile3Line>
where
    O: HasObjectType,
    T: SerializationType,
{
    fn from(obj: Object<T, O>) -> Self {
        let object_type = O::object_type();
        let upn = match obj.sam_account_name() {
            Some(n) => Some(n.to_string()),
            None => obj
                .rdn()
                .as_ref()
                .map(|n| n.name().to_string())
                .or(Some("UNNAMED_OBJECT".to_string())),
        };
        if let Some(upn) = upn {
            vec![
                obj.record_time()
                    .as_ref()
                    .map(|ts| ts.cr_entry(&upn, "record creation time", object_type)),
                obj.when_created()
                    .as_ref()
                    .map(|ts| ts.cr_entry(&upn, "object created", object_type)),
                obj.when_changed()
                    .as_ref()
                    .map(|ts| ts.c_entry(&upn, "object changed", object_type)),
                obj.last_logon()
                    .as_ref()
                    .map(|ts| ts.a_entry(&upn, "last logon on this DC", object_type)),
                obj.last_logon_time_stamp()
                    .as_ref()
                    .map(|ts| ts.c_entry(&upn, "last logon on any DC", object_type)),
                obj.bad_pwd_time()
                    .as_ref()
                    .map(|ts| ts.c_entry(&upn, "bad pwd time", object_type)),
                obj.password_last_set()
                    .as_ref()
                    .map(|ts| ts.m_entry(&upn, "password last set", object_type)),
            ]
            .into_iter()
            .flatten()
            .collect()
        } else {
            Vec::new()
        }
    }
}
