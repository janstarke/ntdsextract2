use libesedb::ColumnVariant;

pub (crate) struct ColumnInformation {
    id: i32,
    name: String,
    variant: ColumnVariant,
}

impl ColumnInformation {
    pub fn new(id: i32, name: String, variant: ColumnVariant) -> Self {
        Self {
            id,
            name,
            variant,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn variant(&self) -> &ColumnVariant {
        &self.variant
    }
}