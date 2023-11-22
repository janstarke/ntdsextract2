use crate::{CTable, ntds::DataTableRecord, RecordHasParent, RecordPredicate};


pub type CDataTable<'r> = CTable<'r>;
impl<'r> CDataTable<'r> {

    pub fn iter_records<'d>(&'d self) -> impl Iterator<Item = DataTableRecord<'d, 'r>> {
        self.iter().map(DataTableRecord::from)
    }

    pub(crate) fn children_of<'d>(&'d self, parent_id: i32) -> impl Iterator<Item = DataTableRecord<'d, 'r>> {
        let my_filter = RecordHasParent(parent_id);
        self.iter_records().filter(move |r| my_filter.matches(r))
    }

    pub fn filter<'d, C>(&'d self, predicate: C) -> impl Iterator<Item = DataTableRecord<'d, 'r>>
    where
        C: Fn(&DataTableRecord<'d, 'r>) -> bool,
    {
        self.iter_records().filter(move |r| predicate(r))
    }

    pub fn find<'d, C>(&'d self, predicate: C) -> Option<DataTableRecord<'d, 'r>>
    where
        C: Fn(&DataTableRecord<'d, 'r>) -> bool,
    {
        self.iter_records().find(move |r| predicate(r))
    }

    pub fn filter_p<'d, P>(&'d self, predicate: P) -> impl Iterator<Item = DataTableRecord<'d, 'r>>
    where
        P: RecordPredicate,
    {
        self.iter_records().filter(move |r| predicate.matches(r))
    }

    pub fn find_p<'d, P>(&'d self, predicate: P) -> Option<DataTableRecord<'d, 'r>>
    where
        P: RecordPredicate,
    {
        self.iter_records().find(move |r| predicate.matches(r))
    }
}

