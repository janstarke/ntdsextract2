use anyhow::anyhow;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::Display,
    hash::Hash,
    rc::{Rc, Weak},
};

use getset::Getters;
use lazy_static::lazy_static;

use crate::{
    cache::{MetaDataCache, RecordPointer, SpecialRecords}, ntds::SdTable, win32_types::Rdn
};
lazy_static! {
    static ref DOMAINROOT_CHILDREN: HashSet<String> = HashSet::from_iter(vec![
        "Deleted Objects".to_string(),
        "Configuration".to_string(),
        "Builtin".to_string(),
        //"DomainDnsZones".to_string(),
        "NTDS Quotas".to_string()
    ].into_iter());
}

/// represents an object in the DIT
#[derive(Getters)]
#[getset(get = "pub")]
pub struct ObjectTreeEntry {
    name: Rdn,
    relative_distinguished_name: String,
    distinguished_name: String,
    record_ptr: RecordPointer,
    sd_id: Option<String>,
    //parent: Option<Weak<ObjectTreeEntry>>,
    children: RefCell<HashSet<Rc<ObjectTreeEntry>>>,
    parent: Option<Weak<Self>>,
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
        let is_deleted = self.name().deleted_from_container().is_some();
        let display_name = self.relative_distinguished_name();
        let sddl = self
            .sd_id()
            .as_ref()
            .map(|sddl| format!(";{sddl}"))
            .unwrap_or_default();

        let flags = if is_deleted { "DELETED; " } else { "" };

        write!(f, "{display_name} ({flags}{}{sddl})", self.record_ptr)
    }
}

impl ObjectTreeEntry {
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

    pub(crate) fn populate_object_tree(
        metadata: &MetaDataCache,
        sd_table: &SdTable,
        record_index: &mut HashMap<RecordPointer, Weak<Self>>,
    ) -> Rc<ObjectTreeEntry> {
        log::info!("populating the object tree");
        Self::create_tree_node(metadata.root(), metadata, sd_table, None, record_index)
    }

    fn create_tree_node(
        record_ptr: &RecordPointer,
        metadata: &MetaDataCache,
        sd_table: &SdTable,
        parent: Option<Weak<Self>>,
        record_index: &mut HashMap<RecordPointer, Weak<Self>>,
    ) -> Rc<ObjectTreeEntry> {
        let entry = &metadata[record_ptr];
        let name = entry.rdn().to_owned();
        let relative_distinguished_name = metadata.rdn(entry);

        let distinguished_name = match &parent {
            Some(parent) => match parent.upgrade() {
                Some(parent) => {
                    if parent.parent.is_none() {
                        log::debug!("hiding the $ROOT_OBJECT$ item");
                        relative_distinguished_name.clone()
                    } else {
                        format!(
                            "{relative_distinguished_name},{}",
                            parent.distinguished_name()
                        )
                    }
                }
                None => {
                    panic!(
                        "unable to upgrade weak link to parent object; there \
                    seems to be an inconsistency in the object tree"
                    );
                }
            },
            None => {
                log::debug!("found the object tree root");
                relative_distinguished_name.clone()
            }
        };

        let sd_id = entry
            .sd_id()
            .as_ref()
            .map(|sd| sd.to_string());

        let me = Rc::new(ObjectTreeEntry {
            name,
            relative_distinguished_name,
            distinguished_name,
            record_ptr: *record_ptr,
            children: RefCell::new(HashSet::new()),
            parent,
            sd_id,
        });

        record_index.insert(*record_ptr, Rc::downgrade(&me));

        // [`ObjectTreeEntry`] uses interior mutability, but its hash()-Implementation
        // don't use the mutable parts, so this is not a problem
        #[allow(clippy::mutable_key_type)]
        let children: HashSet<_> = metadata
            .children_of(record_ptr)
            .map(|c| {
                Self::create_tree_node(
                    c.record_ptr(),
                    metadata,
                    sd_table,
                    Some(Rc::downgrade(&me)),
                    record_index,
                )
            })
            .collect();
        me.children.replace_with(|_| children);
        me
    }

    pub fn get_special_records(root: Rc<ObjectTreeEntry>) -> anyhow::Result<SpecialRecords> {
        log::info!("obtaining special record ids");

        let domain_root =
            ObjectTreeEntry::find_domain_root(&root).ok_or(anyhow!("db has no domain root"))?;

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

    /// returns the path to the domain root object, where the first entry in the list is the domain root object,
    /// and the last object is the root of the tree
    pub fn find_domain_root(root: &Rc<ObjectTreeEntry>) -> Option<Vec<Rc<ObjectTreeEntry>>> {
        let my_children: HashSet<_> = root
            .children()
            .borrow()
            .iter()
            .map(|o| o.name().to_string())
            .collect();

        if my_children.is_superset(&DOMAINROOT_CHILDREN) {
            return Some(vec![Rc::clone(root)]);
        } else {
            for child in root.children().borrow().iter() {
                if let Some(mut path) = Self::find_domain_root(child) {
                    path.push(Rc::clone(root));
                    return Some(path);
                }
            }
        }

        None
    }

    pub fn find_child_by_name(&self, name: &str) -> Option<Rc<ObjectTreeEntry>> {
        self.children()
            .borrow()
            .iter()
            .find(|e| e.name().name() == name)
            .cloned()
    }
}
