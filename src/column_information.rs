// use libesedb::ColumnVariant;

use crate::cache::ColumnIndex;

pub struct ColumnInformation {
    id: ColumnIndex,
    // name: String,
    // variant: ColumnVariant,
}

impl ColumnInformation {
    pub fn new(id: i32, 
        // name: String, 
        // variant: ColumnVariant
    ) -> Self {
        Self {
            id: ColumnIndex::from(id),
            // name,
            // variant,
        }
    }

    pub fn id(&self) -> &ColumnIndex {
        &self.id
    }

    // pub fn name(&self) -> &str {
    //     &self.name
    // }

    // pub fn variant(&self) -> &ColumnVariant {
    //     &self.variant
    // }
}