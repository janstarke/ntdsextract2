use crate::ntds::{Error, NtdsAttributeId, Result};
use crate::value::FromValue;
use crate::{
    win32_types::TruncatedWindowsFileTime,
    CRecord,
};
use concat_idents::concat_idents;
use dashmap::mapref::one::RefMut;
use libesedb::Value;

/// This struct implements only a typed view on a record, but does not hold own data.
pub struct DataTableRecord<'d, 'r>(&'d CRecord<'r>);

impl<'d, 'r> From<&'d CRecord<'r>> for DataTableRecord<'d, 'r> {
    fn from(record: &'d CRecord<'r>) -> Self {
        Self(record)
    }
}

macro_rules! record_attribute {
    ($name: ident, $id: ident, $type: ty) => {
        pub fn $name(&self) -> Result<$type> {
            <$type>::from_value(
                self.0
                    .get_by_id(NtdsAttributeId::$id)
                    .ok_or(Error::ValueIsMissing)?
                    .value(),
            )
        }
        concat_idents!(fn_name = $name, _opt {
            pub fn fn_name (&self) -> Result<Option<$type>> {
                <$type>::from_value_opt(
                    self.0
                        .get_by_id(NtdsAttributeId::$id)
                        .ok_or(Error::ValueIsMissing)?
                        .value(),
                )
            }
        });
    };
}

impl<'d, 'r> DataTableRecord<'d, 'r> {
    record_attribute!(ds_record_id, DsRecordId, i32);
    record_attribute!(ds_parent_record_id, DsParentRecordId, i32);
    record_attribute!(ds_record_time, DsRecordTime, TruncatedWindowsFileTime);
    record_attribute!(ds_ancestors, DsAncestors, i32);
    record_attribute!(ds_object_type_id, AttObjectCategory, i32);
    record_attribute!(ds_object_name, AttCommonName, String);
    record_attribute!(ds_object_name2, AttRdn, String);
    record_attribute!(ds_link_id, AttLinkId, u32);

    pub fn get(&self, attribute_id: NtdsAttributeId) -> Option<RefMut<'_, i32, Value>> {
        self.0.get_by_id(attribute_id)
    }
    pub fn get_by_index(&self, index: i32) -> Option<RefMut<'_, i32, Value>> {
        self.0.get_by_index(index)
    }
}
