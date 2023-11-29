use crate::ntds::DataTableRecord;

pub trait RecordPredicate<'info, 'db> {
    fn matches(&self, record: &DataTableRecord<'info, 'db>) -> bool;
}

pub struct RecordHasId(pub i32);

impl<'info, 'db> RecordPredicate<'info, 'db> for RecordHasId {
    fn matches(&self, record: &DataTableRecord<'info, 'db>) -> bool {
        match record.ds_record_id_opt() {
            Ok(Some(r)) => r == self.0,
            _ => false,
        }
    }
}

pub struct RecordHasRid(pub u32);

impl<'info, 'db> RecordPredicate<'info, 'db> for RecordHasRid {
    fn matches(&self, record: &DataTableRecord<'info, 'db>) -> bool {
        match record.att_object_sid_opt() {
            Ok(Some(sid)) => sid.get_rid() == &self.0,
            _ => false,
        }
    }
}

pub struct RecordHasParent(pub i32);

impl<'info, 'db> RecordPredicate<'info, 'db> for RecordHasParent {
    fn matches(&self, record: &DataTableRecord<'info, 'db>) -> bool {
        match record.ds_parent_record_id_opt() {
            Ok(Some(r)) => r == self.0,
            _ => false,
        }
    }
}

pub struct RecordHasAttRdn(pub &'static str);

impl<'info, 'db> RecordPredicate<'info, 'db> for RecordHasAttRdn {
    fn matches(&self, record: &DataTableRecord<'info, 'db>) -> bool {
        match record.att_object_name2_opt() {
            Ok(Some(r)) => r == self.0,
            _ => false,
        }
    }
}
