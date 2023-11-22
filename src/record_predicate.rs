use crate::ntds::DataTableRecord;

pub trait RecordPredicate {
    fn matches(&self, record: &DataTableRecord) -> bool;
}

pub struct RecordHasId(pub i32);

impl RecordPredicate for RecordHasId {
    fn matches(&self, record: &DataTableRecord) -> bool {
        match record.ds_record_id_opt() {
            Ok(Some(r)) => r == self.0,
            _ => false,
        }
    }
}

pub struct RecordHasParent(pub i32);

impl RecordPredicate for RecordHasParent {
    fn matches(&self, record: &DataTableRecord) -> bool {
        match record.ds_parent_record_id_opt() {
            Ok(Some(r)) => r == self.0,
            _ => false,
        }
    }
}

pub struct RecordHasAttRdn(pub &'static str);

impl RecordPredicate for RecordHasAttRdn {
    fn matches(&self, record: &DataTableRecord) -> bool {
        match record.ds_object_name2_opt() {
            Ok(Some(r)) => r == self.0,
            _ => false,
        }
    }
}
