use std::fmt::Display;

use bodyfile::Bodyfile3Line;

use super::UnixTimestamp;

pub trait TimelineEntry: UnixTimestamp {
    fn cr_entry(&self, upn: &str, caption: &str, object_type: impl Display) -> Bodyfile3Line {
        Bodyfile3Line::new()
            .with_owned_name(format!("{upn} ({object_type}, {caption})"))
            .with_crtime(i64::max(0, self.timestamp()))
    }

    fn c_entry(&self, upn: &str, caption: &str, object_type: impl Display) -> Bodyfile3Line {
        Bodyfile3Line::new()
            .with_owned_name(format!("{upn} ({object_type}, {caption})"))
            .with_ctime(i64::max(0, self.timestamp()))
    }
}