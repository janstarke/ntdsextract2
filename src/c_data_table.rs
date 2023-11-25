use crate::{
    ntds::DataTableRecord, CRecord, CTable, EsedbRecord, RecordHasParent, RecordPredicate,
};

trait DataTableTrait<'table, R>
where
    for<'record> R: EsedbRecord<'record>,
{
}
pub type CDataTable<'table, R>
where
    for<'record> R: EsedbRecord<'record>,
= CTable<'table, R>;

impl<'table, R, CDT> DataTableTrait<'table, R> for CDT
where
    CDT: DataTableTrait<'table, R>,
    R: for<'record> EsedbRecord<'record>,
{
}

trait DataTableIterator<'d, 'r>: Iterator<Item = &'d DataTableRecord<'d, CRecord<'r>>> {}

impl<'table, R> CDataTable<'table, R>
where
    for<'record> R: EsedbRecord<'record>,
{
    pub fn iter<'d, I>(&'d self) -> I
    where
        for<'r> I: DataTableIterator<'d, 'r>,
    {
        self.iter().map(DataTableRecord::from)
    }

    pub(crate) fn children_of<'d, I>(&'d self, parent_id: i32) -> I
    where
        for<'r> I: DataTableIterator<'d, 'r>,
    {
        let my_filter = RecordHasParent(parent_id);
        self.iter_records().filter(move |r| my_filter.matches(r))
    }

    pub fn filter<'d, C, I>(&'d self, predicate: C) -> I
    where
        for<'r> I: DataTableIterator<'d, 'r>,
        C: Fn(&DataTableRecord<'d, R>) -> bool,
    {
        self.iter_records().filter(move |r| predicate(r))
    }

    pub fn find<'d, C>(&'d self, predicate: C) -> Option<DataTableRecord<'d, R>>
    where
        C: Fn(&DataTableRecord<'d, R>) -> bool,
    {
        self.iter_records().find(move |r| predicate(r))
    }

    pub fn filter_p<'d, P, I>(&'d self, predicate: P) -> I
    where
        for<'r> I: DataTableIterator<'d, 'r>,
        for<'r> P: RecordPredicate<'r, R>,
    {
        self.iter_records().filter(move |r| predicate.matches(r))
    }

    pub fn find_p<'d, P>(&'d self, predicate: P) -> Option<DataTableRecord<'d, R>>
    where
    for<'r> P: RecordPredicate<'r, R>,
    {
        self.iter_records().find(move |r| predicate.matches(r))
    }
}
