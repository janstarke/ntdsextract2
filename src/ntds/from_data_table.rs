use serde::Serialize;

use crate::OutputOptions;

use super::{DataTable, DataTableRecord, LinkTable};

pub trait FromDataTable: Sized + Serialize {
    fn new(
        dbrecord: DataTableRecord,
        options: &OutputOptions,
        data_table: &DataTable,
        link_table: &LinkTable,
    ) -> Result<Self, anyhow::Error>;
}
