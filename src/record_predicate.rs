use crate::ntds::DataTableRecord;

pub trait RecordPredicate<'r, R> {
    fn matches(&self, record: &DataTableRecord<'r, R>) -> bool;
}

pub struct RecordHasId(pub i32);

impl<'r, R> RecordPredicate<'r, R> for RecordHasId {
    fn matches(&self, record: &DataTableRecord<'r, R>) -> bool {
        match record.ds_record_id_opt() {
            Ok(Some(r)) => r == self.0,
            _ => false,
        }
    }
}

pub struct RecordHasParent(pub i32);

impl<'r, R> RecordPredicate<'r, R> for RecordHasParent {
    fn matches(&self, record: &DataTableRecord<'r, R>) -> bool {
        match record.ds_parent_record_id_opt() {
            Ok(Some(r)) => r == self.0,
            _ => false,
        }
    }
}

pub struct RecordHasAttRdn(pub &'static str);

impl<'r, R> RecordPredicate<'r, R> for RecordHasAttRdn {
    fn matches(&self, record: &DataTableRecord<'r, R>) -> bool {
        match record.ds_object_name2_opt() {
            Ok(Some(r)) => r == self.0,
            _ => false,
        }
    }
}
