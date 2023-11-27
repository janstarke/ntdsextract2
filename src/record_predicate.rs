use crate::ntds::DataTableRecord;

pub trait RecordPredicate<'t, 'r> {
    fn matches(&self, record: &DataTableRecord<'t, 'r>) -> bool;
}

pub struct RecordHasId(pub i32);

impl<'t, 'r> RecordPredicate<'t, 'r> for RecordHasId {
    fn matches(&self, record: &DataTableRecord<'t, 'r>) -> bool {
        match record.ds_record_id_opt() {
            Ok(Some(r)) => r == self.0,
            _ => false,
        }
    }
}

pub struct RecordHasParent(pub i32);

impl<'t, 'r> RecordPredicate<'t, 'r> for RecordHasParent {
    fn matches(&self, record: &DataTableRecord<'t, 'r>) -> bool {
        match record.ds_parent_record_id_opt() {
            Ok(Some(r)) => r == self.0,
            _ => false,
        }
    }
}

pub struct RecordHasAttRdn(pub &'static str);

impl<'t, 'r> RecordPredicate<'t, 'r> for RecordHasAttRdn {
    fn matches(&self, record: &DataTableRecord<'t, 'r>) -> bool {
        match record.ds_object_name2_opt() {
            Ok(Some(r)) => r == self.0,
            _ => false,
        }
    }
}
