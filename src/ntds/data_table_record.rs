use crate::ntds::NtdsAttributeId;
use crate::value::FromValue;
use crate::{
    value::{ConversionError, ConversionResult},
    win32_types::TruncatedWindowsFileTime,
    CRecord,
};
use concat_idents::concat_idents;
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
        pub fn $name(&self) -> ConversionResult<$type> {
            <$type>::from_value(
                self.0
                    .get(NtdsAttributeId::$id)
                    .ok_or(ConversionError::ValueIsMissing)?,
            )
        }
        concat_idents!(fn_name = $name, _opt {
            pub fn fn_name (&self) -> ConversionResult<Option<$type>> {
                <$type>::from_value_opt(
                    self.0
                        .get(NtdsAttributeId::$id)
                        .ok_or(ConversionError::ValueIsMissing)?,
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
    record_attribute!(ds_object_name, AttCommonName, &str);
    record_attribute!(ds_object_name2, AttRdn, &str);
    record_attribute!(ds_link_id, AttLinkId, u32);

    pub fn get(&self, attribute_id: NtdsAttributeId) -> Option<&Value> {
        self.0.get(attribute_id)
    }
    pub fn get_value_in_column(&self, index: i32) -> Option<&Value> {
        self.0.get_value_in_column(index)
    }
}
