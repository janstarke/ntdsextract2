use crate::cache::RecordId;

pub enum EntryId {
    Id(RecordId),
    Rid(u32),
}