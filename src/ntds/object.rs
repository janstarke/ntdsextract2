use serde::Serialize;

use crate::OutputOptions;

use super::{DataTableRecord, DataTable, LinkTable};

pub trait Object: Serialize + Sized {
    fn new(
        dbrecord: DataTableRecord,
        options: &OutputOptions,
        data_table: &DataTable,
        link_table: &LinkTable,
    ) -> Result<Self, anyhow::Error>;
}
