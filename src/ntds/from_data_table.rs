use serde::Serialize;

use crate::{cli::OutputOptions, FormattedValue};

use super::{DataTable, DataTableRecord, LinkTable};

pub trait FromDataTable: Sized + Serialize {
    fn new(
        dbrecord: DataTableRecord,
        options: &OutputOptions,
        data_table: &DataTable,
        link_table: &LinkTable,
        distinguished_name: FormattedValue<String>
    ) -> Result<Self, anyhow::Error>;
}
