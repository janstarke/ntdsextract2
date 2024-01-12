use std::{ops::Index, collections::HashSet};

use getset::Getters;
use hashbrown::HashMap;

use crate::cache::{MetaDataCache, RecordPointer, SpecialRecords};

use super::ObjectType;

#[derive(Getters)]
#[getset(get="pub")]
pub struct Schema {
    supported_type_entries: HashMap<ObjectType, RecordPointer>,
    all_type_entries: HashSet<RecordPointer>,
}

impl Schema {
    pub fn new(metadata: &MetaDataCache, special_records: &SpecialRecords) -> Self {
        let mut supported_type_entries = HashMap::new();
        let mut all_type_entries = HashSet::new();
        for record in metadata.children_of(special_records.schema().record_ptr()) {
            if let Ok(object_type) = ObjectType::try_from(&record.rdn().name()[..]) {
                supported_type_entries.insert(object_type, *record.record_ptr());
            }
            all_type_entries.insert(*record.record_ptr());
        }
        Self { supported_type_entries, all_type_entries }
    }
}

impl Index<&ObjectType> for Schema {
    type Output = RecordPointer;

    fn index(&self, index: &ObjectType) -> &Self::Output {
        &self.supported_type_entries[index]
    }
}
