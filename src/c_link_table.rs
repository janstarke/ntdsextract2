use crate::{CTable, CRecord};

pub type CLinkTable<'table, 'record> = CTable<'table, CRecord<'record>>;