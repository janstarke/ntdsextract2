use std::{cell::RefCell, collections::HashMap, fmt::Display, hash::Hash, rc::Rc};

use getset::Getters;
use hashbrown::HashSet;
use termtree::Tree;

use crate::cache::{self, DataTable, RecordPointer};
use anyhow::{bail, Result};

/// represents an object in the DIT
#[derive(Getters)]
#[getset(get="pub")]
pub struct ObjectTreeEntry {
    name: String,
    id: RecordPointer,
    //parent: Option<Weak<ObjectTreeEntry>>,
    children: RefCell<HashSet<Rc<ObjectTreeEntry>>>,
}

impl Eq for ObjectTreeEntry {}

impl PartialEq for ObjectTreeEntry {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.id == other.id
    }
}

impl Hash for ObjectTreeEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.id.hash(state);
    }
}

impl Display for ObjectTreeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (id={})", self.name, self.id)
    }
}

impl ObjectTreeEntry {
    pub(crate) fn from(
        data_table: &cache::Table<'_, '_, DataTable>,
    ) -> Result<Rc<ObjectTreeEntry>> {
        Self::populate_object_tree(data_table)
    }

    pub fn get(&self, rdn: &str) -> Option<Rc<Self>> {
        log::debug!("searching for {rdn}");
        for child in self.children.borrow().iter() {
            log::debug!("  candidate is {}", child.name);
            if child.name == rdn {
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
    fn populate_object_tree(
        data_table: &cache::Table<'_, '_, DataTable>,
    ) -> Result<Rc<ObjectTreeEntry>> {
        log::info!("populating the object tree");

        //let mut downlinks = HashMap::new();
        let mut uplinks = HashMap::new();
        let mut names = HashMap::new();

        for record in data_table.iter() {
            let id = record.ds_record_id()?;
            let parent_id = record.ds_parent_record_id()?;
            let name = record.att_object_name2()?.to_owned();

            names.insert(id, name);
            uplinks.insert(id, parent_id.into());
            /*
            downlinks
                .entry(parent_id)
                .or_insert_with(HashSet::new)
                .insert(id);
             */
        }

        log::debug!("found {} entries in the DIT", names.len());
        log::debug!("ordering those elements in a tree structure now");

        Self::create_tree_node(None, &uplinks, &mut names)
    }

    fn create_tree_node(
        object_id: Option<&RecordPointer>,
        uplinks: &HashMap<RecordPointer, RecordPointer>,
        names: &mut HashMap<RecordPointer, String>,
    ) -> Result<Rc<ObjectTreeEntry>> {
        match object_id {
            None => {
                let mut children = uplinks
                    .iter()
                    .filter(|(_, parent)| parent.ds_record_id().inner() == 0);
                match children.next() {
                    None => bail!("missing root object"),
                    Some((root, _)) => {
                        if children.next().is_some() {
                            bail!("more than one root object found");
                        }
                        Self::create_tree_node(Some(root), uplinks, names)
                    }
                }
            }

            Some(object_id) => {
                let name = names
                    .get(&object_id)
                    .ok_or_else(|| {
                        anyhow::anyhow!("missing name for object with id '{object_id}'")
                    })?
                    .to_owned();

                let children: Result<HashSet<Rc<Self>>> = uplinks
                    .iter()
                    .filter(|(_, parent)| *parent == object_id)
                    .map(|(c, _)| Self::create_tree_node(Some(c), uplinks, names))
                    .collect();
                Ok(Rc::new(ObjectTreeEntry {
                    name,
                    id: *object_id,
                    //parent: parent.and_then(|p| Some(Rc::downgrade(p))),
                    children: RefCell::new(children?),
                }))
            }
        }
    }
}
