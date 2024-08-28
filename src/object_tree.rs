use anyhow::anyhow;
use std::{
    collections::HashMap,
    rc::{Rc, Weak},
};
use termtree::Tree;

use crate::{
    cache::{MetaDataCache, RecordPointer, SpecialRecords},
    object_tree_entry::ObjectTreeEntry,
};

pub struct ObjectTree {
    root: Rc<ObjectTreeEntry>,
    record_index: HashMap<RecordPointer, Weak<ObjectTreeEntry>>,
}

impl ObjectTree {
    pub fn new(metadata: &MetaDataCache) -> Self {
        let mut record_index = HashMap::new();
        let root = ObjectTreeEntry::populate_object_tree(metadata, &mut record_index);
        Self { root, record_index }
    }

    pub fn get_special_records(&self) -> anyhow::Result<SpecialRecords> {
        log::info!("obtaining special record ids");

        let domain_root = ObjectTreeEntry::find_domain_root(&self.root)
            .ok_or(anyhow!("db has no domain root"))?;

        log::info!("found domain root '{}'", domain_root[0].name());

        let configuration = domain_root[0]
            .find_child_by_name("Configuration")
            .ok_or(anyhow!("db has no `Configuration` entry"))?;

        let schema_subpath = configuration
            .find_child_by_name("Schema")
            .ok_or(anyhow!("db has no `Schema` entry"))?;

        let deleted_objects = domain_root[0]
            .find_child_by_name("Deleted Objects")
            .ok_or(anyhow!("db has no `Deleted Objects` entry"))?;

        Ok(SpecialRecords::new(schema_subpath, deleted_objects))
    }

    pub(crate) fn to_termtree(&self, max_depth: u8) -> Tree<Rc<ObjectTreeEntry>> {
        Self::__to_termtree(&self.root, max_depth)
    }

    pub fn __to_termtree(me: &Rc<ObjectTreeEntry>, max_depth: u8) -> Tree<Rc<ObjectTreeEntry>> {
        let tree = Tree::new(Rc::clone(me));
        if max_depth > 0 {
            let leaves: Vec<Tree<Rc<ObjectTreeEntry>>> = me
                .children()
                .borrow()
                .iter()
                .map(|c| Self::__to_termtree(c, max_depth - 1))
                .collect();
            tree.with_leaves(leaves)
        } else {
            tree
        }
    }

    pub fn dn_of(&self, ptr: &RecordPointer) -> String {
        self.record_index
            .get(ptr)
            .unwrap_or_else(|| panic!("invalid record pointer: {ptr}"))
            .upgrade()
            .unwrap_or_else(|| panic!("record pointer {ptr} points to already deleted object"))
            .distinguished_name()
            .clone()
    }
}
