use anyhow::anyhow;
use std::{cell::RefCell, fmt::Display, hash::Hash, rc::Rc};

use getset::Getters;
use hashbrown::HashSet;
use termtree::Tree;

use crate::{
    cache::{MetaDataCache, RecordPointer, SpecialRecords},
    win32_types::Rdn,
};

/// represents an object in the DIT
#[derive(Getters)]
#[getset(get = "pub")]
pub struct ObjectTreeEntry {
    name: Rdn,
    record_ptr: RecordPointer,
    //parent: Option<Weak<ObjectTreeEntry>>,
    children: RefCell<HashSet<Rc<ObjectTreeEntry>>>,
}

impl Eq for ObjectTreeEntry {}

impl PartialEq for ObjectTreeEntry {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.record_ptr == other.record_ptr
    }
}

impl Hash for ObjectTreeEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.record_ptr.hash(state);
    }
}

impl Display for ObjectTreeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.name().guid() {
            Some(_) => write!(f, "{} (DELETED; {})", self.name.name(), self.record_ptr),
            None => write!(f, "{} ({})", self.name.name(), self.record_ptr),
        }
    }
}

impl ObjectTreeEntry {
    pub(crate) fn from(metadata: &MetaDataCache) -> Rc<ObjectTreeEntry> {
        Self::populate_object_tree(metadata)
    }

    pub fn get(&self, rdn: &str) -> Option<Rc<Self>> {
        log::debug!("searching for {rdn}");
        for child in self.children.borrow().iter() {
            log::debug!("  candidate is {}", child.name);
            if child.name.name() == rdn {
                return Some(Rc::clone(child));
            }
        }
        None
    }

    pub(crate) fn to_tree(me: &Rc<ObjectTreeEntry>, max_depth: u8) -> Tree<Rc<ObjectTreeEntry>> {
        let tree = Tree::new(Rc::clone(me));
        if max_depth > 0 {
            let leaves: Vec<Tree<Rc<ObjectTreeEntry>>> = me
                .children
                .borrow()
                .iter()
                .map(|c| Self::to_tree(c, max_depth - 1))
                .collect();
            tree.with_leaves(leaves)
        } else {
            tree
        }
    }
    /*
        pub(crate) fn parent(&self) -> Option<Rc<ObjectTreeEntry>> {
            self.parent.as_ref().and_then(|p| p.upgrade())
        }
    */
    /*
        pub (crate) fn get_by_path(&self, mut path: Vec<&str>) -> Option<Rc<ObjectTreeEntry>> {
            if let Some(next_folder) = path.pop() {
                match self.children.borrow().iter().find(|c| c.name == next_folder) {
                    None => None,
                    Some(child) => {
                        if path.len() == 0 {
                            Some(Rc::clone(child))
                        } else {
                            Self::get_by_path(&self, path)
                        }
                    }
                }
            } else {
                None
            }
        }
    */
    fn populate_object_tree(metadata: &MetaDataCache) -> Rc<ObjectTreeEntry> {
        log::info!("populating the object tree");
        Self::create_tree_node(metadata.root(), metadata)
    }

    fn create_tree_node(
        record_ptr: &RecordPointer,
        metadata: &MetaDataCache,
    ) -> Rc<ObjectTreeEntry> {
        let name = metadata[record_ptr].rdn().to_owned();
        let children = metadata
            .children_of(record_ptr)
            .map(|c| Self::create_tree_node(c.record_ptr(), metadata))
            .collect();
        Rc::new(ObjectTreeEntry {
            name,
            record_ptr: *record_ptr,
            children: RefCell::new(children),
        })
    }

    pub fn get_special_records(root: Rc<ObjectTreeEntry>) -> anyhow::Result<SpecialRecords> {
        log::info!("obtaining special record ids");

        // search downward until we find a `Configuration` entry
        let configuration_path = ObjectTreeEntry::find_first_in_tree(&root, "Configuration")
            .ok_or(anyhow!("db has no `Configuration` entry"))?;

        let schema_subpath = configuration_path[0]
            .find_child_by_name("Schema")
            .ok_or(anyhow!("db has no `Schema` entry"))?;

        let deleted_objects_subpath = configuration_path[0]
            .find_child_by_name("Deleted Objects")
            .ok_or(anyhow!("db has no `Deleted Objects` entry"))?;

        Ok(SpecialRecords::new(schema_subpath, deleted_objects_subpath))
    }

    pub fn find_first_in_tree(
        root: &Rc<ObjectTreeEntry>,
        name: &str,
    ) -> Option<Vec<Rc<ObjectTreeEntry>>> {
        if root.name().name() == name {
            Some(vec![Rc::clone(root)])
        } else {
            for child in root.children().borrow().iter() {
                if let Some(mut path) = Self::find_first_in_tree(child, name) {
                    path.push(Rc::clone(root));
                    return Some(path);
                }
            }
            None
        }
    }

    pub fn find_child_by_name(&self, name: &str) -> Option<Rc<ObjectTreeEntry>> {
        self.children()
            .borrow()
            .iter()
            .find(|e| e.name().name() == name)
            .map(Rc::clone)
    }
}
