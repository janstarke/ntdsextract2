use std::collections::{HashMap, HashSet};
use std::ops::Index;

use getset::Getters;
use lazy_static::lazy_static;

use crate::value::FromValue;
use crate::win32_types::Sid;
use crate::{ntds::NtdsAttributeId, EsedbInfo};

use super::{EsedbRowId, RecordId, RecordPointer, Value};

#[derive(Getters)]
#[getset(get = "pub")]
pub struct DataEntryCore {
    record_ptr: RecordPointer,
    parent: RecordId,
    object_category: Option<RecordId>,
    rdn: String,
    sid: Option<Sid>,
}

lazy_static! {
    static ref EMPTY_HASHSET: HashSet<RecordPointer> = HashSet::new();
}

#[derive(Getters)]
pub struct MetaDataCache {
    records: Vec<DataEntryCore>,
    record_rows: HashMap<RecordId, RecordPointer>,
    children_of: HashMap<RecordId, HashSet<RecordPointer>>,

    #[getset(get = "pub")]
    root: RecordPointer,
}

impl TryFrom<&EsedbInfo<'_>> for MetaDataCache {
    type Error = anyhow::Error;
    fn try_from(info: &EsedbInfo<'_>) -> Result<Self, Self::Error> {
        let record_id_column = **info.mapping().index(NtdsAttributeId::DsRecordId).id();
        let parent_column = **info.mapping().index(NtdsAttributeId::DsParentRecordId).id();
        let rdn_column = **info.mapping().index(NtdsAttributeId::AttRdn).id();
        let object_category_column = **info.mapping().index(NtdsAttributeId::AttObjectCategory).id();
        let sid_column = **info.mapping().index(NtdsAttributeId::AttObjectSid).id();

        let mut records = Vec::new();
        let mut record_rows = HashMap::new();
        let mut children_of: HashMap<RecordId, HashSet<RecordPointer>> = HashMap::new();
        let mut root = None;
        let count = info.data_table().count_records()?;
        let bar = crate::create_progressbar("Creating cache for record IDs".to_string(), count.try_into()?)?;
        for esedb_row_id in 0..count {
            let record = info.data_table().record(esedb_row_id)?;
            let parent = match RecordId::from_value_opt(&Value::from(record.value(parent_column)?))? {
                Some(v) => v,
                None => continue
            };
            let record_id = match RecordId::from_value_opt(&Value::from(record.value(record_id_column)?))? {
                Some(v) => v,
                None => continue
            };
            let rdn = match String::from_value_opt(&Value::from(record.value(rdn_column)?))? {
                Some(v) => v,
                None => continue
            };
            let object_category = RecordId::from_value_opt(&Value::from(record.value(object_category_column)?))?;
            let sid = Sid::from_value_opt(&Value::from(record.value(sid_column)?))?;

            let record_ptr = RecordPointer::new(record_id, esedb_row_id.into());

            records.push(DataEntryCore {
                record_ptr,
                parent,
                //cn,
                rdn,
                object_category,
                sid
            });

            record_rows.insert(record_id, RecordPointer::new(record_id, esedb_row_id.into()));


            if parent.inner() != 0 {
                children_of.entry(parent).or_default().insert(record_ptr);
            } else {
                if root.is_some() {
                    panic!("more than one root object found");
                } else {
                    root = Some(record_ptr);
                }
            }

            bar.inc(1);
        }
        bar.finish_and_clear();

        Ok(Self {
            records,
            record_rows,
            children_of,
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

impl Index<&RecordId> for MetaDataCache {
    type Output = DataEntryCore;

    fn index(&self, index: &RecordId) -> &Self::Output {
        let ptr = self.record_rows[index];
        &self.records[ptr.esedb_row().inner() as usize]
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
        self.children_of
            .get(&parent.ds_record_id())
            .unwrap_or(&EMPTY_HASHSET)
            .iter()
            .map(|ptr| &self[ptr.esedb_row()])
    }

    pub fn entries_with_rid(&self, rid: u32)  -> impl Iterator<Item = &DataEntryCore> + '_ {
        self.records.iter().filter(move |r| match r.sid() {
            Some(sid) => sid.get_rid() == &rid,
            _ => false
        })
    }

    pub fn entries_of_type(&self, ot: &RecordId) -> impl Iterator<Item = &DataEntryCore> + '_ {
        let ot = *ot;
        self.records.iter().filter(move |r| match r.object_category() {
            Some(oc) => *oc == ot,
            _ => false
        })
    }

    pub fn entries_of_types(&self, ot: HashSet<RecordId>) -> impl Iterator<Item = &DataEntryCore> + '_ {
        self.records.iter().filter(move |r| match r.object_category() {
            Some(oc) => ot.contains(oc),
            _ => false
        })
    }

    pub fn ptr_from_row(&self, row: &EsedbRowId) -> &RecordPointer {
        self[row].record_ptr()
    }

    pub fn ptr_from_id(&self, id: &RecordId) -> &RecordPointer {
        &self.record_rows[id]
    }
}
