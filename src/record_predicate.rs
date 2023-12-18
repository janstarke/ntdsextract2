use crate::{cache::RecordPointer, ntds::DataTableRecord};

pub trait RecordPredicate<'info, 'db> {
    fn matches(&self, record: &DataTableRecord<'info, 'db>) -> bool;
}

pub struct RecordHasRid(pub u32);

impl<'info, 'db> RecordPredicate<'info, 'db> for RecordHasRid {
    fn matches(&self, record: &DataTableRecord<'info, 'db>) -> bool {
        match record.att_object_sid() {
            Ok(sid) => sid.get_rid() == &self.0,
            _ => false,
        }
    }
}

pub struct RecordHasParent(pub RecordPointer);

impl<'info, 'db> RecordPredicate<'info, 'db> for RecordHasParent {
    fn matches(&self, record: &DataTableRecord<'info, 'db>) -> bool {
        log::debug!(
            "searching children of {}; current is {:?}",
            self.0,
            record.ds_parent_record_id()
        );
        match record.ds_parent_record_id() {
            Ok(r) => r == self.0,
            _ => false,
        }
    }
}

pub struct RecordHasAttRdn(pub &'static str);

impl<'info, 'db> RecordPredicate<'info, 'db> for RecordHasAttRdn {
    fn matches(&self, record: &DataTableRecord<'info, 'db>) -> bool {
        match record.att_object_name2() {
            Ok(r) => r == self.0,
            _ => false,
        }
    }
}
