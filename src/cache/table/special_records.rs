use std::rc::Rc;

use getset::Getters;

use crate::object_tree_entry::ObjectTreeEntry;

#[derive(Getters)]
#[getset(get = "pub", set = "pub")]
pub struct SpecialRecords {
    schema: Rc<ObjectTreeEntry>,
    deleted_objects: Rc<ObjectTreeEntry>,
}

impl SpecialRecords {
    pub fn new(schema: Rc<ObjectTreeEntry>, deleted_objects: Rc<ObjectTreeEntry>) -> Self {
        Self {
            schema,
            deleted_objects,
        }
    }
}
