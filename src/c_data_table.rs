use crate::{CTable, ntds::DataTableRecord, RecordHasParent, RecordPredicate, CRecord};


pub type CDataTable<'table, 'record> = CTable<'table, CRecord<'record>>;
impl<'table, 'record> CDataTable<'table, 'record> {

    pub fn iter_records<'d>(&'d self) -> impl Iterator<Item = DataTableRecord<'d, 'record>> {
        self.iter().map(DataTableRecord::from)
    }

    pub(crate) fn children_of<'d>(&'d self, parent_id: i32) -> impl Iterator<Item = DataTableRecord<'d, 'record>> {
        let my_filter = RecordHasParent(parent_id);
        self.iter_records().filter(move |r| my_filter.matches(r))
    }

    pub fn filter<'d, C>(&'d self, predicate: C) -> impl Iterator<Item = DataTableRecord<'d, 'record>>
    where
        C: Fn(&DataTableRecord<'d, 'record>) -> bool,
    {
        self.iter_records().filter(move |r| predicate(r))
    }

    pub fn find<'d, C>(&'d self, predicate: C) -> Option<DataTableRecord<'d, 'record>>
    where
        C: Fn(&DataTableRecord<'d, 'record>) -> bool,
    {
        self.iter_records().find(move |r| predicate(r))
    }

    pub fn filter_p<'d, P>(&'d self, predicate: P) -> impl Iterator<Item = DataTableRecord<'d, 'record>>
    where
        P: RecordPredicate,
    {
        self.iter_records().filter(move |r| predicate.matches(r))
    }

    pub fn find_p<'d, P>(&'d self, predicate: P) -> Option<DataTableRecord<'d, 'record>>
    where
        P: RecordPredicate,
    {
        self.iter_records().find(move |r| predicate.matches(r))
    }
}

