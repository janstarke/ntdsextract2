use bodyfile::Bodyfile3Line;

use crate::ntds::ObjectType;

use super::UnixTimestamp;

pub trait TimelineEntry: UnixTimestamp {
    fn cr_entry(&self, upn: &str, caption: &str, object_type: ObjectType) -> Bodyfile3Line {
        Bodyfile3Line::new()
            .with_owned_name(format!("{upn} ({object_type}, {caption})"))
            .with_crtime(i64::max(0, self.timestamp()))
    }

    fn c_entry(&self, upn: &str, caption: &str, object_type: ObjectType) -> Bodyfile3Line {
        Bodyfile3Line::new()
            .with_owned_name(format!("{upn} ({object_type}, {caption})"))
            .with_ctime(i64::max(0, self.timestamp()))
    }
}