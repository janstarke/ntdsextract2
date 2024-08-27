use crate::cache::RecordPointer;
use crate::cli::OutputOptions;
use crate::win32_types::{Rdn, TimelineEntry, TruncatedWindowsFileTime, WindowsFileTime};
use crate::win32_types::{SamAccountType, Sid, UserAccountControl};
use crate::{RdnSet, SerializationType};
use bodyfile::Bodyfile3Line;
use getset::Getters;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};

use crate::ntds::{DataTable, DataTableRecord, FromDataTable, HasObjectType, LinkTable};
use std::marker::PhantomData;

use super::{HasSerializableFields, SpecificObjectAttributes};

#[derive(Getters, Deserialize)]
#[getset(get = "pub")]
#[serde(bound = "T: SerializationType")]
pub struct Object<T, O, A>
where
    O: HasObjectType,
    T: SerializationType,
    A: SpecificObjectAttributes,
{
    distinguished_name: Option<String>,

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

    //#[serde(flatten)]
    specific_attributes: A,

    #[serde(skip)]
    _marker: PhantomData<O>,

    #[serde(skip)]
    ptr: RecordPointer,
}

impl<T, O, A> HasSerializableFields for Object<T, O, A> 
where
    O: HasObjectType,
    T: SerializationType,
    A: SpecificObjectAttributes,
{
    fn fields() -> &'static Vec<&'static str> {
        lazy_static::lazy_static!{
            static ref FIELDS: Vec<&'static str> = vec![
                "sid",
                "user_principal_name",
                "rdn",
                "sam_account_name",
                "sam_account_type",
                "user_account_control",
                "logon_count",
                "bad_pwd_count",
                "admin_count",
                "is_deleted",
                "primary_group_id",
                "primary_group",
                "member_of",
                "comment",
                "record_time",
                "when_created",
                "when_changed",
                "last_logon",
                "last_logon_time_stamp",
                "account_expires",
                "password_last_set",
                "bad_pwd_time"
            ];
        }
        &FIELDS
    }
}

impl<T, O, A> Serialize for Object<T, O, A>
where
    O: HasObjectType,
    T: SerializationType,
    A: SpecificObjectAttributes,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Object", Self::field_count() + A::field_count())?;

        s.serialize_field("sid", self.sid())?;
        
        if let Some(dn) = &self.distinguished_name {
            s.serialize_field("distinguished_name", dn)?;
        }

        s.serialize_field("user_principal_name", self.user_principal_name())?;
        s.serialize_field("rdn", self.rdn())?;
        s.serialize_field("sam_account_name", self.sam_account_name())?;
        s.serialize_field("sam_account_type", self.sam_account_type())?;
        s.serialize_field("user_account_control", self.user_account_control())?;
        s.serialize_field("logon_count", self.logon_count())?;
        s.serialize_field("bad_pwd_count", self.bad_pwd_count())?;
        s.serialize_field("admin_count", self.admin_count())?;
        s.serialize_field("is_deleted", self.is_deleted())?;
        s.serialize_field("primary_group_id", self.primary_group_id())?;
        s.serialize_field("primary_group", self.primary_group())?;
        s.serialize_field("member_of", self.member_of())?;
        s.serialize_field("comment", self.comment())?;
        s.serialize_field("record_time", self.record_time())?;
        s.serialize_field("when_created", self.when_created())?;
        s.serialize_field("when_changed", self.when_changed())?;
        s.serialize_field("last_logon", self.last_logon())?;
        s.serialize_field("last_logon_time_stamp", self.last_logon_time_stamp())?;
        s.serialize_field("account_expires", self.account_expires())?;
        s.serialize_field("password_last_set", self.password_last_set())?;
        s.serialize_field("bad_pwd_time", self.bad_pwd_time())?;

        self.specific_attributes().serialize_to::<S>(&mut s)?;
        s.end()
    }
}

impl<T, O, A> FromDataTable for Object<T, O, A>
where
    O: HasObjectType,
    T: SerializationType,   
    A: SpecificObjectAttributes,
{
    fn new(
        dbrecord: DataTableRecord,
        _options: &OutputOptions,
        data_table: &DataTable,
        link_table: &LinkTable,
        distinguished_name: Option<String>,
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
                        .data_table_record_from(*e.record_ptr())
                        .unwrap()
                })
                .map(|group| group.att_object_name2().unwrap())
        });

        let member_of = link_table.member_names_of(object_id, data_table).into();
        let specific_attributes = A::from(&dbrecord)?;

        Ok(Self {
            distinguished_name,
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
            specific_attributes,
            _marker: PhantomData,
            ptr: *dbrecord.ptr(),
        })
    }
}

impl<T, O, A> From<Object<T, O, A>> for Vec<Bodyfile3Line>
where
    O: HasObjectType,
    T: SerializationType,
    A: SpecificObjectAttributes,
{
    fn from(obj: Object<T, O, A>) -> Self {
        let object_type = O::object_type();
        let upn = match obj.sam_account_name() {
            Some(n) => Some(n.to_string()),
            None => obj
                .rdn()
                .as_ref()
                .map(|n| n.name().to_string())
                .or(Some("UNNAMED_OBJECT".to_string())),
        };
        let inode = obj.ptr().ds_record_id().to_string();

        if let Some(upn) = upn {
            vec![
                obj.record_time().as_ref().map(|ts| {
                    ts.cr_entry(&upn, "record creation time", object_type)
                        .with_inode(&inode)
                }),
                obj.when_created().as_ref().map(|ts| {
                    ts.cr_entry(&upn, "object created", object_type)
                        .with_inode(&inode)
                }),
                obj.when_changed().as_ref().map(|ts| {
                    ts.c_entry(&upn, "object changed", object_type)
                        .with_inode(&inode)
                }),
                obj.last_logon().as_ref().map(|ts| {
                    ts.a_entry(&upn, "last logon on this DC", object_type)
                        .with_inode(&inode)
                }),
                obj.last_logon_time_stamp().as_ref().map(|ts| {
                    ts.c_entry(&upn, "last logon on any DC", object_type)
                        .with_inode(&inode)
                }),
                obj.bad_pwd_time().as_ref().map(|ts| {
                    ts.c_entry(&upn, "bad pwd time", object_type)
                        .with_inode(&inode)
                }),
                obj.password_last_set().as_ref().map(|ts| {
                    ts.m_entry(&upn, "password last set", object_type)
                        .with_inode(&inode)
                }),
            ]
            .into_iter()
            .flatten()
            .collect()
        } else {
            Vec::new()
        }
    }
}
