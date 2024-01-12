use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::ops::Index;

use anyhow::bail;
use getset::Getters;
use lazy_static::lazy_static;

use crate::value::FromValue;
use crate::win32_types::{Rdn, Sid, Guid};
use crate::{ntds::NtdsAttributeId, EsedbInfo};

use super::{EsedbRowId, RecordId, RecordPointer};

#[derive(Getters)]
#[getset(get = "pub")]
pub struct DataEntryCore {
    record_ptr: RecordPointer,
    parent: RecordId,
    object_category: Option<RecordId>,
    cn: Option<Rdn>,
    rdn: Rdn,
    sid: Option<Sid>,
    rdn_typ_col: Option<i32>,
    

    relative_distinguished_name: Option<Rdn>,

    #[getset(skip)]
    distinguished_name: RefCell<Option<String>>,
}

impl Display for DataEntryCore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.rdn.name(), self.record_ptr)
    }
}

lazy_static! {
    static ref EMPTY_HASHSET: HashSet<RecordPointer> = HashSet::new();
}

#[derive(Getters)]
pub struct MetaDataCache {
    records: Vec<DataEntryCore>,
    record_rows: HashMap<RecordId, RecordPointer>,
    children_of: HashMap<RecordId, HashSet<RecordPointer>>,

    #[getset(skip)]
    record_by_guid: HashMap<Guid, RecordPointer>,
    attributes: HashMap<i32, String>,

    #[getset(get = "pub")]
    root: RecordPointer,
}

impl TryFrom<&EsedbInfo<'_>> for MetaDataCache {
    type Error = anyhow::Error;
    fn try_from(info: &EsedbInfo<'_>) -> Result<Self, Self::Error> {
        let record_id_column = *info.mapping().index(NtdsAttributeId::DsRecordId).id();
        let parent_column = *info.mapping().index(NtdsAttributeId::DsParentRecordId).id();
        let rdn_column = *info.mapping().index(NtdsAttributeId::AttRdn).id();
        let cn_column = *info.mapping().index(NtdsAttributeId::AttCommonName).id();
        let object_category_column = *info
            .mapping()
            .index(NtdsAttributeId::AttObjectCategory)
            .id();
        let sid_column = *info.mapping().index(NtdsAttributeId::AttObjectSid).id();
        let guid_column = *info.mapping().index(NtdsAttributeId::AttObjectGuid).id();
        /*let rdn_att_id = *info
            .mapping()
            .info_by_name("RDNtyp_col")
            .expect("missing column 'RDNtyp_col'")
            .id();
        */
        let rdn_att_id = *info.mapping().index(NtdsAttributeId::AttRdnAttId).id();
        let attribute_id_column = *info.mapping().index(NtdsAttributeId::AttAttributeId).id();
        let ldap_display_name_column = *info
            .mapping()
            .index(NtdsAttributeId::AttLdapDisplayName)
            .id();

        let mut records = Vec::new();
        let mut record_rows = HashMap::new();
        let mut children_of: HashMap<RecordId, HashSet<RecordPointer>> = HashMap::new();
        let mut attributes = HashMap::new();
        let mut record_by_guid = HashMap::new();
        let mut root = None;
        //let mut root_dse = None;
        let count = info.data_table().count_records()?;
        let bar = crate::create_progressbar(
            "Creating cache for record IDs".to_string(),
            count.try_into()?,
        )?;

        for esedb_row_id in 0..count {
            let record = info.data_table().record(esedb_row_id)?;

            if let Some(parent) = RecordId::from_record_opt(&record, parent_column)? {
                if let Some(record_id) = RecordId::from_record_opt(&record, record_id_column)? {
                    if let Some(rdn) = Rdn::from_record_opt(&record, rdn_column)? {
                        let cn = Rdn::from_record_opt(&record, cn_column)?;
                        let object_category =
                            RecordId::from_record_opt(&record, object_category_column)?;
                        let sid = Sid::from_record_opt(&record, sid_column)?;
                        let guid = Guid::from_record_opt(&record, guid_column)?;

                        if let Some(attribute_id) =
                            i32::from_record_opt(&record, attribute_id_column)?
                        {
                            if let Some(ldap_display_name) =
                                String::from_record_opt(&record, ldap_display_name_column)?
                            {
                                if attributes.contains_key(&attribute_id) {
                                    bail!("unambigious attribute id: {attribute_id} in {record_id}")
                                } else {
                                    attributes.insert(attribute_id, ldap_display_name);
                                }
                            }
                        }

                        let rdn_typ_col = i32::from_record_opt(&record, rdn_att_id)?;
                        let rdn_val_col = match rdn_typ_col {
                            Some(id) => {
                                let column_name = format!("ATTm{id}");
                                match info.mapping().info_by_name(&column_name[..]) {
                                    Some(id) => *id.id(),
                                    None => {
                                        log::error!("invalid column name: '{column_name}', using 'cn' instead");
                                        cn_column
                                    }
                                }
                            }
                            None => cn_column,
                        };
                        let relative_distinguished_name =
                            Rdn::from_record_opt(&record, rdn_val_col)?;

                        let record_ptr = RecordPointer::new(record_id, esedb_row_id.into());

                        if parent.inner() != 0 {
                            children_of.entry(parent).or_default().insert(record_ptr);
                        } else if root.is_some() {
                            panic!("object without parent: '{rdn}' at '{record_ptr}");
                        } else {
                            // check if this really is the root entry
                            if rdn.name() == "$ROOT_OBJECT$" {
                                root = Some(record_ptr);
                            } else {
                                log::warn!("object without parent: '{rdn}' at '{record_ptr}");
                            }
                        }

                        records.push(DataEntryCore {
                            record_ptr,
                            parent,
                            rdn,
                            cn,
                            object_category,
                            sid,
                            rdn_typ_col,
                            relative_distinguished_name,
                            distinguished_name: RefCell::new(None),
                        });

                        record_rows.insert(
                            record_id,
                            RecordPointer::new(record_id, esedb_row_id.into()),
                        );

                        if let Some(guid) = guid {
                            record_by_guid.insert(guid, RecordPointer::new(record_id, esedb_row_id.into()));
                        }
                    } else {
                        log::warn!(
                            "ignoring entry in row {esedb_row_id}: attribute {} (RDN) has no value",
                            Into::<&str>::into(NtdsAttributeId::AttRdn)
                        )
                    }
                } else {
                    log::warn!(
                        "ignoring entry in row {esedb_row_id}: attribute {} (RecordID) has no value",
                        Into::<&str>::into(NtdsAttributeId::DsRecordId)
                    )
                }
            } else {
                log::warn!(
                    "ignoring entry in row {esedb_row_id}: attribute {} (ParentRecordId) has no value",
                    Into::<&str>::into(NtdsAttributeId::DsParentRecordId)
                )
            }

            bar.inc(1);
        }
        bar.finish_and_clear();

        Ok(Self {
            records,
            record_rows,
            children_of,
            attributes,
            record_by_guid,
            root: root.expect("no root object found"),
        })
    }
}

impl Index<&EsedbRowId> for MetaDataCache {
    type Output = DataEntryCore;

    fn index(&self, index: &EsedbRowId) -> &Self::Output {
        &self.records[index.inner() as usize]
    }
}

impl Index<&RecordPointer> for MetaDataCache {
    type Output = DataEntryCore;

    fn index(&self, index: &RecordPointer) -> &Self::Output {
        &self[index.esedb_row()]
    }
}

impl MetaDataCache {
    pub fn iter(&self) -> impl Iterator<Item = &DataEntryCore> {
        self.records.iter()
    }

    pub fn children_of(&self, parent: &RecordPointer) -> impl Iterator<Item = &DataEntryCore> {
        self.children_ptr_of(parent)
            .map(|ptr| &self[ptr.esedb_row()])
    }

    pub fn children_ptr_of(&self, parent: &RecordPointer) -> impl Iterator<Item = &RecordPointer> {
        self.children_of
            .get(parent.ds_record_id())
            .unwrap_or(&EMPTY_HASHSET)
            .iter()
    }

    pub fn entries_with_rid(&self, rid: u32) -> impl Iterator<Item = &DataEntryCore> + '_ {
        self.records.iter().filter(move |r| match r.sid() {
            Some(sid) => sid.get_rid() == &rid,
            _ => false,
        })
    }

    pub fn entries_of_type(&self, ot: &RecordId) -> impl Iterator<Item = &DataEntryCore> + '_ {
        let ot = *ot;
        self.records
            .iter()
            .filter(move |r| match r.object_category() {
                Some(oc) => *oc == ot,
                _ => false,
            })
    }

    pub fn entries_of_types(
        &self,
        ot: HashSet<RecordId>,
    ) -> impl Iterator<Item = &DataEntryCore> + '_ {
        self.records
            .iter()
            .filter(move |r| match r.object_category() {
                Some(oc) => ot.contains(oc),
                _ => false,
            })
    }

    pub fn entries_with_deleted_from_container_guid(&self) -> impl Iterator<Item = &RecordPointer> {
        self.records
            .iter()
            .filter(|r| r.rdn().deleted_from_container().is_some())
            .map(|d| &d.record_ptr)
    }

    pub fn ptr_from_row(&self, row: &EsedbRowId) -> &RecordPointer {
        self[row].record_ptr()
    }

    pub fn ptr_from_id(&self, id: &RecordId) -> Option<&RecordPointer> {
        self.record_rows.get(id)
    }

    pub fn record(&self, index: &RecordId) -> Option<&DataEntryCore> {
        match self.record_rows.get(index) {
            Some(ptr) => self.records.get(ptr.esedb_row().inner() as usize),
            None => None,
        }
    }

    pub fn ptr_from_guid(&self, guid: &Guid) -> Option<&RecordPointer> {
        self.record_by_guid.get(guid)
    }

    pub fn rdn(&self, entry: &DataEntryCore) -> String {
        if let Some(type_entry_id) = entry.object_category() {
            if let Some(type_entry) = self.record(type_entry_id) {
                if let Some(rdn_att_id) = type_entry.rdn_typ_col() {
                    if let Some(ldap_display_name) = self.attributes.get(&rdn_att_id) {
                        return format!("{ldap_display_name}={}", entry.rdn().name());
                    } else {
                        log::warn!("no record entry found for attribute id {rdn_att_id}");
                    }
                } else {
                    log::warn!(
                        "no attribute id found for {entry} (object category is {type_entry})"
                    );
                }
            } else {
                log::warn!("invalid object category for {entry}: {type_entry_id}");
            }
        } else {
            log::warn!("no object category for {entry}");
        }

        return format!("cn={}", entry.cn().as_ref().unwrap_or(&entry.rdn()).name());
    }

    pub fn dn(&self, entry: &DataEntryCore) -> Option<String> {
        if entry.parent.inner() == 0 {
            None
        } else if let Some(dn) = entry.distinguished_name.borrow().as_ref() {
            Some(dn.to_string())
        } else {
            let rdn = self.rdn(entry);
            match self.dn(self
                .record(&entry.parent)
                .expect("invalid parent reference"))
            {
                Some(parent_dn) => Some(format!("{rdn},{parent_dn}")),
                None => Some(rdn),
            }
            .map(|dn| {
                //let _ = entry.distinguished_name.replace(Some(dn.to_string()));
                dn
            })
        }
    }
}
